use core::convert::TryInto;
use crate::blocks::flash;
use crate::blocks::flash::{FlashExt, UnlockedFlash, MemIter};

#[derive(Copy, Clone)]
enum Page {
    Page0,
    Page1,
}

impl Page {
    fn id(&self) -> u8 {
        match self {
            Page::Page0 => 0,
            Page::Page1 => 1,
        }
    }

    fn start_address(&self) -> usize {
        match self {
            Page::Page0 => 0x0800_4000,
            Page::Page1 => 0x0800_8000,
        }
    }

    const fn size() -> usize {
        0x3fff
    }

    const fn cell_count() -> usize {
        (Self::size() - 2) / 6
    }

    fn sector(&self) -> u8 {
        match self {
            Page::Page0 => 1,
            Page::Page1 => 2,
        }
    }

    fn next(&self) -> Page {
        match self {
            Page::Page0 => Page::Page1,
            Page::Page1 => Page::Page0,
        }
    }
}

const ACTIVE_PAGE_MARKER: u16 = 0xbeef;
const FLASH_START: usize = 0x0800_0000;
const HEADER_SIZE: usize = 2;
const CELL_SIZE: usize = 6;
const EMPTY_KEY: u16 = 0xffff;

pub struct Storage {
    flash: stm32f4xx_hal::stm32::FLASH,
}

impl Storage {
    pub fn new(flash: stm32f4xx_hal::stm32::FLASH) -> Storage {
        Self { flash }
    }

    pub fn init(&mut self) -> Result<(), flash::Error> {
        let active_page = self.find_active_page();
        if let Some(active_page) = active_page {
            defmt::info!(
                "Found active page on address: {:x}",
                active_page.start_address()
            );
        } else {
            defmt::info!("Didn't find active page. Formatting");
            self.erase()?;
        }
        Ok(())
    }

    pub fn erase(&mut self) -> Result<(), flash::Error> {
        let mut unlocked = self.flash.unlocked();
        Self::format(&mut unlocked)?;
        Self::mark_active_page(&mut unlocked, &Page::Page0)
    }

    pub fn read(&self, key: u16) -> Option<u32> {
        let active_page = self
            .find_active_page()
            .expect("Failed to find active page.");
        self.find_by_key(&active_page, key)
    }

    pub fn read_f32(&self, key: u16) -> Option<f32> {
        self.read(key).map(|u| unsafe { *(u.to_le_bytes().as_ptr() as *const f32) })
    }

    pub fn write_raw(&mut self, key: u16, raw: &[u8]) -> Result<(), flash::Error> {
        let integer = raw.as_ptr() as *const u32;
        self.write(key, unsafe { *integer })
    }

    pub fn write_f32(&mut self, key: u16, value: f32) -> Result<(), flash::Error> {
        self.write_raw(key, &value.to_le_bytes())
    }

    pub fn write(&mut self, key: u16, value: u32) -> Result<(), flash::Error> {
        let active_page = self
            .find_active_page()
            .expect("Failed to access the active page.");
        self.move_to_new_page_if_needed()?;
        for cell in 0..Page::cell_count() {
            if self.cell_key_value(&active_page, cell).0 == EMPTY_KEY {
                defmt::error!("found empty cell {}, writing", cell);
                Self::write_cell(&mut self.flash.unlocked(), &active_page, cell, key, value)?;
                break;
            }
        }
        Ok(())
    }

    fn find_active_page(&self) -> Option<Page> {
        for page in &[Page::Page0, Page::Page1] {
            let header = self.read_page_header(page);
            if header == ACTIVE_PAGE_MARKER {
                return Some(*page);
            }
        }
        None
    }

    fn mark_active_page(flash: &mut UnlockedFlash, page: &Page) -> Result<(), flash::Error> {
        defmt::info!("Marking page {} active.", page.id());
        let bytes = ACTIVE_PAGE_MARKER.to_le_bytes();
        let iter = MemIter::new(bytes);
        flash.program(page.start_address() - FLASH_START, iter)
    }

    fn find_by_key(&self, page: &Page, key: u16) -> Option<u32> {
        for cell in (0..Page::cell_count()).rev() {
            let (cell_key, value) = self.cell_key_value(page, cell);
            if cell_key == key {
                defmt::info!("Key {:x} found in the cell {:x}", key, cell);
                return Some(value);
            }
        }
        None
    }

    fn cell_key_value(&self, page: &Page, cell: usize) -> (u16, u32) {
        let address = page.start_address() - FLASH_START + HEADER_SIZE + cell * CELL_SIZE;
        let memory = self.flash.read();
        (
            u16::from_le_bytes(memory[address + 4..address + CELL_SIZE].try_into().unwrap()),
            u32::from_le_bytes(memory[address..address + 4].try_into().unwrap()),
        )
    }

    fn move_to_new_page_if_needed(&mut self) -> Result<(), flash::Error> {
        defmt::info!("Checking for empty space in a page.");
        let active_page = self
            .find_active_page()
            .expect("Failed to access the active page.");
        if self.cell_key_value(&active_page, Page::cell_count() - 1).0 == EMPTY_KEY {
            defmt::info!("There is still room in the current page.");
            return Ok(());
        }
        defmt::info!("Moving data between pages.");
        let target_page = active_page.next();
        let mut target_cell = 0;
        for cell in (0..Page::cell_count()).rev() {
            let (key, value) = self.cell_key_value(&active_page, cell);
            if key == EMPTY_KEY {
                continue;
            }
            if self.find_by_key(&target_page, key).is_none() {
                Self::write_cell(
                    &mut self.flash.unlocked(),
                    &target_page,
                    target_cell,
                    key,
                    value,
                )?;
                target_cell += 1;
            }
        }
        defmt::info!("Erasing page: {}", active_page.id());
        let mut unlocked = self.flash.unlocked();
        unlocked.erase(active_page.sector())?;
        Self::mark_active_page(&mut unlocked, &target_page)
    }

    fn write_cell(
        flash: &mut UnlockedFlash,
        page: &Page,
        cell: usize,
        key: u16,
        value: u32,
    ) -> Result<(), flash::Error> {
        defmt::info!(
            "Writing cell {:x} of page {}, key: {:x}, value: {:x}",
            cell,
            page.id(),
            key,
            value
        );

        let offset = page.start_address() - FLASH_START + HEADER_SIZE + cell * CELL_SIZE;

        let iter = MemIter::new(value.to_le_bytes());
        flash.program(offset, iter)?;

        let iter = MemIter::new(key.to_le_bytes());
        flash.program(offset + 4, iter)
    }

    fn format(flash: &mut UnlockedFlash) -> Result<(), flash::Error> {
        defmt::info!("Formatting flash.");
        flash.erase(Page::Page0.sector())?;
        flash.erase(Page::Page1.sector())
    }

    fn read_page_header(&self, page: &Page) -> u16 {
        let start = page.start_address() - FLASH_START;
        u16::from_le_bytes(self.flash.read()[start..start + 2].try_into().unwrap())
    }
}
