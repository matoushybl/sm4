var searchIndex = JSON.parse('{\
"main":{"doc":"","i":[[0,"init","main","Initialization function",null,null],[3,"Schedule","main::init","Tasks that can be <code>schedule</code>-d from this context",null,null],[12,"_not_send","","",0,null],[3,"LateResources","","Resources initialized at runtime",null,null],[12,"driver","","",1,null],[3,"Context","","Execution context",null,null],[12,"start","","System start time = <code>Instant(0 /* cycles */)</code>",2,null],[12,"core","","Core (Cortex-M) peripherals minus the SysTick",2,null],[12,"device","","Device peripherals",2,null],[12,"schedule","","Tasks that can be <code>schedule</code>-d from this context",2,null],[11,"new","","",2,[[["peripherals",3]]]],[0,"main","main","Idle loop",null,null],[3,"Resources","main::main","Resources <code>main</code> has access to",null,null],[12,"driver","","",3,null],[3,"Context","","Execution context",null,null],[12,"resources","","Resources this task has access to",4,null],[11,"new","","",4,[[["priority",3]]]],[0,"resources","main","",null,null],[3,"driver","main::resources","",null,null],[12,"priority","","",5,null],[11,"new","","",5,[[["priority",3]]]],[11,"priority","","",5,[[],["priority",3]]],[0,"usb_handler","main","Hardware task",null,null],[3,"Resources","main::usb_handler","Resources <code>usb_handler</code> has access to",null,null],[12,"driver","","",6,null],[3,"Context","","Execution context",null,null],[12,"start","","Time at which this handler started executing",7,null],[12,"resources","","Resources this task has access to",7,null],[11,"new","","",7,[[["priority",3]]]],[0,"dma","main","Hardware task",null,null],[3,"Resources","main::dma","Resources <code>dma</code> has access to",null,null],[12,"driver","","",8,null],[3,"Context","","Execution context",null,null],[12,"start","","Time at which this handler started executing",9,null],[12,"resources","","Resources this task has access to",9,null],[11,"new","","",9,[[["priority",3]]]],[0,"can_handler","main","Hardware task",null,null],[3,"Resources","main::can_handler","Resources <code>can_handler</code> has access to",null,null],[12,"driver","","",10,null],[3,"Context","","Execution context",null,null],[12,"start","","Time at which this handler started executing",11,null],[12,"resources","","Resources this task has access to",11,null],[11,"new","","",11,[[["priority",3]]]],[0,"blink","main","Software task",null,null],[3,"Resources","main::blink","Resources <code>blink</code> has access to",null,null],[12,"driver","","",12,null],[3,"Schedule","","Tasks that can be <code>schedule</code>-d from this context",null,null],[12,"priority","","",13,null],[3,"Context","","Execution context",null,null],[12,"scheduled","","The time at which this task was scheduled to run",14,null],[12,"resources","","Resources this task has access to",14,null],[12,"schedule","","Tasks that can be <code>schedule</code>-d from this context",14,null],[11,"new","","",14,[[["priority",3]]]],[0,"monitoring","main","Software task",null,null],[3,"Resources","main::monitoring","Resources <code>monitoring</code> has access to",null,null],[12,"driver","","",15,null],[3,"Schedule","","Tasks that can be <code>schedule</code>-d from this context",null,null],[12,"priority","","",16,null],[3,"Context","","Execution context",null,null],[12,"scheduled","","The time at which this task was scheduled to run",17,null],[12,"resources","","Resources this task has access to",17,null],[12,"schedule","","Tasks that can be <code>schedule</code>-d from this context",17,null],[11,"new","","",17,[[["priority",3]]]],[0,"control","main","Software task",null,null],[3,"Resources","main::control","Resources <code>control</code> has access to",null,null],[12,"driver","","",18,null],[3,"Schedule","","Tasks that can be <code>schedule</code>-d from this context",null,null],[12,"priority","","",19,null],[3,"Context","","Execution context",null,null],[12,"scheduled","","The time at which this task was scheduled to run",20,null],[12,"resources","","Resources this task has access to",20,null],[12,"schedule","","Tasks that can be <code>schedule</code>-d from this context",20,null],[11,"new","","",20,[[["priority",3]]]],[0,"ramp","main","Software task",null,null],[3,"Resources","main::ramp","Resources <code>ramp</code> has access to",null,null],[12,"driver","","",21,null],[3,"Schedule","","Tasks that can be <code>schedule</code>-d from this context",null,null],[12,"priority","","",22,null],[3,"Context","","Execution context",null,null],[12,"scheduled","","The time at which this task was scheduled to run",23,null],[12,"resources","","Resources this task has access to",23,null],[12,"schedule","","Tasks that can be <code>schedule</code>-d from this context",23,null],[11,"new","","",23,[[["priority",3]]]],[0,"failsafe_tick","main","Software task",null,null],[3,"Resources","main::failsafe_tick","Resources <code>failsafe_tick</code> has access to",null,null],[12,"driver","","",24,null],[3,"Schedule","","Tasks that can be <code>schedule</code>-d from this context",null,null],[12,"priority","","",25,null],[3,"Context","","Execution context",null,null],[12,"scheduled","","The time at which this task was scheduled to run",26,null],[12,"resources","","Resources this task has access to",26,null],[12,"schedule","","Tasks that can be <code>schedule</code>-d from this context",26,null],[11,"new","","",26,[[["priority",3]]]],[0,"heartbeat_tick","main","Software task",null,null],[3,"Resources","main::heartbeat_tick","Resources <code>heartbeat_tick</code> has access to",null,null],[12,"driver","","",27,null],[3,"Schedule","","Tasks that can be <code>schedule</code>-d from this context",null,null],[12,"priority","","",28,null],[3,"Context","","Execution context",null,null],[12,"scheduled","","The time at which this task was scheduled to run",29,null],[12,"resources","","Resources this task has access to",29,null],[12,"schedule","","Tasks that can be <code>schedule</code>-d from this context",29,null],[11,"new","","",29,[[["priority",3]]]],[5,"init","main","",null,[[["context",3]],["lateresources",3]]],[5,"main","","",null,[[["context",3]]]],[5,"usb_handler","","",null,[[["context",3]]]],[5,"dma","","",null,[[["context",3]]]],[5,"can_handler","","",null,[[["context",3]]]],[5,"blink","","",null,[[["context",3]]]],[5,"monitoring","","",null,[[["context",3]]]],[5,"control","","",null,[[["context",3]]]],[5,"ramp","","",null,[[["context",3]]]],[5,"failsafe_tick","","",null,[[["context",3]]]],[5,"heartbeat_tick","","",null,[[["context",3]]]],[3,"initLateResources","","Resources initialized at runtime",null,null],[12,"driver","","",1,null],[3,"mainResources","","Resources <code>main</code> has access to",null,null],[12,"driver","","",3,null],[3,"usb_handlerResources","","Resources <code>usb_handler</code> has access to",null,null],[12,"driver","","",6,null],[3,"dmaResources","","Resources <code>dma</code> has access to",null,null],[12,"driver","","",8,null],[3,"can_handlerResources","","Resources <code>can_handler</code> has access to",null,null],[12,"driver","","",10,null],[3,"blinkResources","","Resources <code>blink</code> has access to",null,null],[12,"driver","","",12,null],[3,"monitoringResources","","Resources <code>monitoring</code> has access to",null,null],[12,"driver","","",15,null],[3,"controlResources","","Resources <code>control</code> has access to",null,null],[12,"driver","","",18,null],[3,"rampResources","","Resources <code>ramp</code> has access to",null,null],[12,"driver","","",21,null],[3,"failsafe_tickResources","","Resources <code>failsafe_tick</code> has access to",null,null],[12,"driver","","",24,null],[3,"heartbeat_tickResources","","Resources <code>heartbeat_tick</code> has access to",null,null],[12,"driver","","",27,null],[17,"APP","","Implementation details",null,null],[11,"from","main::init","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"into","","",0,[[]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","main","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"into","","",1,[[]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","main::init","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"into","","",2,[[]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","main","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"into","","",3,[[]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","main::main","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"into","","",4,[[]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","main::resources","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"into","","",5,[[]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","main","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"into","","",6,[[]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","main::usb_handler","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"into","","",7,[[]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","main","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"into","","",8,[[]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","main::dma","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"into","","",9,[[]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"from","main","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"into","","",10,[[]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","main::can_handler","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"into","","",11,[[]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"from","main","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"into","","",12,[[]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"from","main::blink","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"into","","",13,[[]]],[11,"try_into","","",13,[[],["result",4]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"from","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"into","","",14,[[]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"from","main","",15,[[]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"into","","",15,[[]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"from","main::monitoring","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"into","","",16,[[]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"from","","",17,[[]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"into","","",17,[[]]],[11,"try_into","","",17,[[],["result",4]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"from","main","",18,[[]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"into","","",18,[[]]],[11,"try_into","","",18,[[],["result",4]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"from","main::control","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"into","","",19,[[]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"from","","",20,[[]]],[11,"borrow","","",20,[[]]],[11,"borrow_mut","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"into","","",20,[[]]],[11,"try_into","","",20,[[],["result",4]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"from","main","",21,[[]]],[11,"borrow","","",21,[[]]],[11,"borrow_mut","","",21,[[]]],[11,"try_from","","",21,[[],["result",4]]],[11,"into","","",21,[[]]],[11,"try_into","","",21,[[],["result",4]]],[11,"type_id","","",21,[[],["typeid",3]]],[11,"from","main::ramp","",22,[[]]],[11,"borrow","","",22,[[]]],[11,"borrow_mut","","",22,[[]]],[11,"try_from","","",22,[[],["result",4]]],[11,"into","","",22,[[]]],[11,"try_into","","",22,[[],["result",4]]],[11,"type_id","","",22,[[],["typeid",3]]],[11,"from","","",23,[[]]],[11,"borrow","","",23,[[]]],[11,"borrow_mut","","",23,[[]]],[11,"try_from","","",23,[[],["result",4]]],[11,"into","","",23,[[]]],[11,"try_into","","",23,[[],["result",4]]],[11,"type_id","","",23,[[],["typeid",3]]],[11,"from","main","",24,[[]]],[11,"borrow","","",24,[[]]],[11,"borrow_mut","","",24,[[]]],[11,"try_from","","",24,[[],["result",4]]],[11,"into","","",24,[[]]],[11,"try_into","","",24,[[],["result",4]]],[11,"type_id","","",24,[[],["typeid",3]]],[11,"from","main::failsafe_tick","",25,[[]]],[11,"borrow","","",25,[[]]],[11,"borrow_mut","","",25,[[]]],[11,"try_from","","",25,[[],["result",4]]],[11,"into","","",25,[[]]],[11,"try_into","","",25,[[],["result",4]]],[11,"type_id","","",25,[[],["typeid",3]]],[11,"from","","",26,[[]]],[11,"borrow","","",26,[[]]],[11,"borrow_mut","","",26,[[]]],[11,"try_from","","",26,[[],["result",4]]],[11,"into","","",26,[[]]],[11,"try_into","","",26,[[],["result",4]]],[11,"type_id","","",26,[[],["typeid",3]]],[11,"from","main","",27,[[]]],[11,"borrow","","",27,[[]]],[11,"borrow_mut","","",27,[[]]],[11,"try_from","","",27,[[],["result",4]]],[11,"into","","",27,[[]]],[11,"try_into","","",27,[[],["result",4]]],[11,"type_id","","",27,[[],["typeid",3]]],[11,"from","main::heartbeat_tick","",28,[[]]],[11,"borrow","","",28,[[]]],[11,"borrow_mut","","",28,[[]]],[11,"try_from","","",28,[[],["result",4]]],[11,"into","","",28,[[]]],[11,"try_into","","",28,[[],["result",4]]],[11,"type_id","","",28,[[],["typeid",3]]],[11,"from","","",29,[[]]],[11,"borrow","","",29,[[]]],[11,"borrow_mut","","",29,[[]]],[11,"try_from","","",29,[[],["result",4]]],[11,"into","","",29,[[]]],[11,"try_into","","",29,[[],["result",4]]],[11,"type_id","","",29,[[],["typeid",3]]],[11,"clone","main::init","",0,[[],["schedule",3]]],[11,"clone","main::blink","",13,[[],["schedule",3]]],[11,"clone","main::monitoring","",16,[[],["schedule",3]]],[11,"clone","main::control","",19,[[],["schedule",3]]],[11,"clone","main::ramp","",22,[[],["schedule",3]]],[11,"clone","main::failsafe_tick","",25,[[],["schedule",3]]],[11,"clone","main::heartbeat_tick","",28,[[],["schedule",3]]],[11,"lock","main::resources","",5,[[]]]],"p":[[3,"Schedule"],[3,"initLateResources"],[3,"Context"],[3,"mainResources"],[3,"Context"],[3,"driver"],[3,"usb_handlerResources"],[3,"Context"],[3,"dmaResources"],[3,"Context"],[3,"can_handlerResources"],[3,"Context"],[3,"blinkResources"],[3,"Schedule"],[3,"Context"],[3,"monitoringResources"],[3,"Schedule"],[3,"Context"],[3,"controlResources"],[3,"Schedule"],[3,"Context"],[3,"rampResources"],[3,"Schedule"],[3,"Context"],[3,"failsafe_tickResources"],[3,"Schedule"],[3,"Context"],[3,"heartbeat_tickResources"],[3,"Schedule"],[3,"Context"]]},\
"sm4_firmware":{"doc":"","i":[[0,"blocks","sm4_firmware","",null,null],[0,"current_reference","sm4_firmware::blocks","",null,null],[3,"CurrentDACChannel","sm4_firmware::blocks::current_reference","",null,null],[12,"channel","","",0,null],[11,"new","","",0,[[]]],[5,"initialize_current_ref","","",null,[[["currentref1pin",6],["dac",3],["currentref2pin",6]]]],[0,"gpio","sm4_firmware::blocks","",null,null],[3,"GPIO","sm4_firmware::blocks::gpio","",null,null],[12,"dir1","","",1,null],[12,"dir2","","",1,null],[12,"mode1","","",1,null],[12,"mode2","","",1,null],[12,"ref1","","",1,null],[12,"ref2","","",1,null],[12,"en1","","",1,null],[12,"en2","","",1,null],[12,"step1","","",1,null],[12,"step2","","",1,null],[12,"err1","","",1,null],[12,"err2","","",1,null],[12,"battery_voltage","","",1,null],[12,"error_led","","",1,null],[12,"status_led","","",1,null],[12,"can_rx","","",1,null],[12,"can_tx","","",1,null],[12,"scl","","",1,null],[12,"sda","","",1,null],[12,"usb_minus","","",1,null],[12,"usb_plus","","",1,null],[11,"configure","","",1,[[["parts",3],["parts",3],["parts",3]]]],[0,"leds","sm4_firmware::blocks","",null,null],[3,"LEDs","sm4_firmware::blocks::leds","",null,null],[12,"status_led","","",2,null],[12,"error_led","","",2,null],[11,"new","","",2,[[["statusled",6],["errorled",6]]]],[11,"tick","","",2,[[]]],[11,"signalize_sync","","",2,[[]]],[0,"monitoring","sm4_firmware::blocks","",null,null],[3,"Monitoring","sm4_firmware::blocks::monitoring","",null,null],[12,"transfer","","",3,null],[12,"temperature","","",3,null],[12,"battery_voltage","","",3,null],[12,"transfer_ongoing","","",3,null],[11,"new","","",3,[[["batteryvoltage",6],["dma2",3],["adc1",3],["stream0",3]]]],[11,"poll","","",3,[[]]],[11,"transfer_complete","","",3,[[]]],[11,"get_temperature","","",3,[[],["f32",15]]],[11,"get_battery_voltage","","",3,[[],["f32",15]]],[0,"step_counter","sm4_firmware::blocks","",null,null],[8,"Counter","sm4_firmware::blocks::step_counter","",null,null],[10,"get_value","","",4,[[],["u32",15]]],[10,"reset_value","","",4,[[]]],[3,"StepCounterEncoder","","",null,null],[12,"timer","","",5,null],[12,"past_position","","",5,null],[12,"current_position","","",5,null],[12,"current_velocity","","",5,null],[12,"direction","","",5,null],[12,"sampling_period","","",5,null],[12,"past_value","","",5,null],[11,"update_current_position","","",5,[[]]],[11,"tim2","","",5,[[["microseconds",3],["tim2",3]]]],[11,"tim5","","",5,[[["microseconds",3],["tim5",3]]]],[0,"step_timer","sm4_firmware::blocks","",null,null],[3,"StepGeneratorTimer","sm4_firmware::blocks::step_timer","",null,null],[12,"timer","","",6,null],[12,"clocks","","",6,null],[12,"frequency","","",6,null],[11,"init_tim1","","",6,[[["tim1",3],["clocks",3]]]],[11,"init_tim8","","",6,[[["tim8",3],["clocks",3]]]],[0,"usb","sm4_firmware::blocks","",null,null],[3,"USBProtocol","sm4_firmware::blocks::usb","",null,null],[12,"serial","","",7,null],[12,"usb_dev","","",7,null],[11,"new","","",7,[[["usbdminus",6],["otg_fs_global",3],["otg_fs_device",3],["usbdplus",6],["clocks",3],["otg_fs_pwrclk",3]]]],[11,"process_interrupt","","",7,[[]]],[0,"eeprom","sm4_firmware::blocks","",null,null],[4,"Page","sm4_firmware::blocks::eeprom","",null,null],[13,"Page0","","",8,null],[13,"Page1","","",8,null],[11,"id","","",8,[[],["u8",15]]],[11,"start_address","","",8,[[],["usize",15]]],[11,"size","","",8,[[],["usize",15]]],[11,"cell_count","","",8,[[],["usize",15]]],[11,"sector","","",8,[[],["u8",15]]],[11,"next","","",8,[[],["page",4]]],[17,"ACTIVE_PAGE_MARKER","","",null,null],[17,"FLASH_START","","",null,null],[17,"HEADER_SIZE","","",null,null],[17,"CELL_SIZE","","",null,null],[17,"EMPTY_KEY","","",null,null],[3,"Storage","","",null,null],[12,"flash","","",9,null],[11,"new","","",9,[[["flash",3]],["storage",3]]],[11,"init","","",9,[[],[["error",4],["result",4]]]],[11,"erase","","",9,[[],[["error",4],["result",4]]]],[11,"read","","",9,[[["u16",15]],[["u32",15],["option",4]]]],[11,"read_f32","","",9,[[["u16",15]],[["option",4],["f32",15]]]],[11,"write_raw","","",9,[[["u16",15]],[["error",4],["result",4]]]],[11,"write_f32","","",9,[[["u16",15],["f32",15]],[["error",4],["result",4]]]],[11,"write","","",9,[[["u32",15],["u16",15]],[["error",4],["result",4]]]],[11,"find_active_page","","",9,[[],[["page",4],["option",4]]]],[11,"mark_active_page","","",9,[[["page",4],["unlockedflash",3]],[["error",4],["result",4]]]],[11,"find_by_key","","",9,[[["page",4],["u16",15]],[["u32",15],["option",4]]]],[11,"cell_key_value","","",9,[[["page",4],["usize",15]]]],[11,"move_to_new_page_if_needed","","",9,[[],[["error",4],["result",4]]]],[11,"write_cell","","",9,[[["page",4],["usize",15],["u16",15],["u32",15],["unlockedflash",3]],[["error",4],["result",4]]]],[11,"format","","",9,[[["unlockedflash",3]],[["error",4],["result",4]]]],[11,"read_page_header","","",9,[[["page",4]],["u16",15]]],[0,"flash","sm4_firmware::blocks","",null,null],[3,"MemIter","sm4_firmware::blocks::flash","",null,null],[12,"data","","",10,null],[12,"index","","",10,null],[11,"new","","",10,[[]]],[4,"Error","","Flash erase/program error",null,null],[13,"ProgrammingSequence","","",11,null],[13,"ProgrammingParallelism","","",11,null],[13,"ProgrammingAlignment","","",11,null],[13,"WriteProtection","","",11,null],[13,"Operation","","",11,null],[11,"read","","",11,[[["flash",3]],["option",4]]],[8,"FlashExt","","Flash methods implemented for <code>stm32::FLASH</code>",null,null],[10,"address","","Memory-mapped address",12,[[],["usize",15]]],[10,"len","","Size in bytes",12,[[],["usize",15]]],[11,"read","","Returns a read-only view of flash memory",12,[[]]],[10,"unlocked","","Unlock flash for erasing/programming until this method\'s …",12,[[],["unlockedflash",3]]],[17,"PSIZE_X8","","",null,null],[3,"UnlockedFlash","","Result of <code>FlashExt::unlocked()</code>",null,null],[12,"flash","","",13,null],[11,"erase","","Erase a flash sector",13,[[["u8",15]],[["error",4],["result",4]]]],[11,"program","","Program bytes with offset into flash memory, aligned to …",13,[[["usize",15]],[["error",4],["result",4]]]],[11,"ok","","",13,[[],[["error",4],["result",4]]]],[11,"wait_ready","","",13,[[]]],[17,"UNLOCK_KEY1","","",null,null],[17,"UNLOCK_KEY2","","",null,null],[5,"unlock","","",null,[[["flash",3]]]],[5,"lock","","",null,[[["flash",3]]]],[0,"board","sm4_firmware","",null,null],[0,"config","sm4_firmware::board","",null,null],[17,"CAN_ID","sm4_firmware::board::config","",null,null],[17,"SENSE_R","","",null,null],[17,"MICROSTEPS","","",null,null],[17,"STEPS_PER_REV","","",null,null],[17,"ENCODER_RESOLUTION","","",null,null],[0,"definitions","sm4_firmware::board","",null,null],[6,"Dir1","sm4_firmware::board::definitions","",null,null],[6,"Dir2","","",null,null],[6,"Mode1","","",null,null],[6,"Mode2","","",null,null],[6,"CurrentRef1Pin","","",null,null],[6,"CurrentRef1Channel","","",null,null],[6,"CurrentRef2Pin","","",null,null],[6,"CurrentRef2Channel","","",null,null],[6,"En1","","",null,null],[6,"En2","","",null,null],[6,"Step1","","",null,null],[6,"Step2","","",null,null],[6,"Err1","","",null,null],[6,"Err2","","",null,null],[6,"BatteryVoltage","","",null,null],[6,"ErrorLED","","",null,null],[6,"StatusLED","","",null,null],[6,"CANRx","","",null,null],[6,"CANTx","","",null,null],[6,"SCL","","",null,null],[6,"SDA","","",null,null],[6,"USBDMinus","","",null,null],[6,"USBDPlus","","",null,null],[6,"Axis1Driver","","",null,null],[6,"Axis2Driver","","",null,null],[6,"Axis1Encoder","","",null,null],[6,"Axis2Encoder","","",null,null],[6,"Axis1","","",null,null],[6,"Axis2","","",null,null],[0,"can","sm4_firmware","",null,null],[6,"BUS","sm4_firmware::can","",null,null],[3,"CANOpen","","",null,null],[12,"bus","","",14,null],[12,"id","","",14,null],[11,"new","","",14,[[["can1",3],["can",3],["u8",15]]]],[11,"process_incoming_frame","","",14,[[],["option",4]]],[11,"send","","",14,[[["canopenmessage",4]]]],[4,"CANOpenMessage","","",null,null],[13,"NMTNodeControl","","",15,null],[13,"GlobalFailsafeCommand","","",15,null],[13,"Sync","","",15,null],[13,"Emergency","","",15,null],[13,"TimeStamp","","",15,null],[13,"TxPDO1","","",15,null],[13,"RxPDO1","","",15,null],[13,"TxPDO2","","",15,null],[13,"RxPDO2","","",15,null],[13,"TxPDO3","","",15,null],[13,"RxPDO3","","",15,null],[13,"TxPDO4","","",15,null],[13,"RxPDO4","","",15,null],[13,"TxSDO","","",15,null],[13,"RxSDO","","",15,null],[13,"NMTNodeMonitoring","","",15,null],[11,"message_id_with_device","","",15,[[["u8",15]],["standardid",3]]],[8,"CANOpenFrame","","",null,null],[10,"parse_id","","",16,[[],[["option",4],["canopenmessage",4]]]],[0,"object_dictionary","sm4_firmware","",null,null],[3,"ObjectDictionary","sm4_firmware::object_dictionary","The object dictionary struct represents the global state …",null,null],[12,"battery_voltage","","",17,null],[12,"temperature","","",17,null],[12,"axis1","","",17,null],[12,"axis2","","",17,null],[11,"new","","",17,[[]]],[11,"axis1","","",17,[[],["axisdictionary",3]]],[11,"axis1_mut","","",17,[[],["axisdictionary",3]]],[11,"axis2","","",17,[[],["axisdictionary",3]]],[11,"axis2_mut","","",17,[[],["axisdictionary",3]]],[11,"battery_voltage","","",17,[[],["f32",15]]],[11,"temperature","","",17,[[],["f32",15]]],[11,"set_battery_voltage","","",17,[[["f32",15]]]],[11,"set_temperature","","",17,[[["f32",15]]]],[0,"sm4","sm4_firmware","",null,null],[17,"SECOND","sm4_firmware::sm4","",null,null],[3,"SM4","","",null,null],[12,"leds","","",18,null],[12,"usb","","",18,null],[12,"can","","",18,null],[12,"monitoring","","",18,null],[12,"state","","",18,null],[12,"axis1","","",18,null],[12,"axis2","","",18,null],[11,"init","","",18,[[["peripherals",3]]]],[11,"control","","",18,[[]]],[11,"ramp","","",18,[[]]],[11,"failsafe_tick","","",18,[[]]],[11,"heartbeat_tick","","",18,[[]]],[11,"blink_leds","","",18,[[]]],[11,"monitor","","",18,[[]]],[11,"monitoring_complete","","",18,[[]]],[11,"process_usb","","",18,[[]]],[11,"process_can","","",18,[[]]],[11,"blink_period","","",18,[[],["u32",15]]],[11,"monitoring_period","","",18,[[],["u32",15]]],[11,"ramping_period","","",18,[[],["u32",15]]],[11,"control_period","","",18,[[],["u32",15]]],[11,"failsafe_tick_period","","",18,[[],["u32",15]]],[11,"heartbeat_tick_period","","",18,[[],["u32",15]]],[0,"state","sm4_firmware","",null,null],[17,"SPEED_COMMAND_RESET_INTERVAL","sm4_firmware::state","",null,null],[3,"DriverState","","",null,null],[12,"nmt_state","","",19,null],[12,"object_dictionary","","",19,null],[12,"last_received_speed_command_down_counter","","",19,null],[11,"new","","",19,[[]]],[11,"nmt_state","","",19,[[],["nmtstate",4]]],[11,"go_to_preoperational_if_needed","","",19,[[]]],[11,"go_to_operational","","",19,[[]]],[11,"go_to_stopped","","",19,[[]]],[11,"go_to_preoperational","","",19,[[]]],[11,"is_movement_blocked","","",19,[[],["bool",15]]],[11,"decrement_last_received_speed_command_counter","","",19,[[]]],[11,"invalidate_last_received_speed_command_counter","","",19,[[]]],[11,"object_dictionary","","",19,[[],["objectdictionary",3]]],[0,"prelude","sm4_firmware","",null,null],[0,"config","sm4_firmware::prelude","",null,null],[17,"CAN_ID","sm4_firmware::prelude::config","",null,null],[17,"SENSE_R","","",null,null],[17,"MICROSTEPS","","",null,null],[17,"STEPS_PER_REV","","",null,null],[17,"ENCODER_RESOLUTION","","",null,null],[0,"definitions","sm4_firmware::prelude","",null,null],[6,"Dir1","sm4_firmware::prelude::definitions","",null,null],[6,"Dir2","","",null,null],[6,"Mode1","","",null,null],[6,"Mode2","","",null,null],[6,"CurrentRef1Pin","","",null,null],[6,"CurrentRef1Channel","","",null,null],[6,"CurrentRef2Pin","","",null,null],[6,"CurrentRef2Channel","","",null,null],[6,"En1","","",null,null],[6,"En2","","",null,null],[6,"Step1","","",null,null],[6,"Step2","","",null,null],[6,"Err1","","",null,null],[6,"Err2","","",null,null],[6,"BatteryVoltage","","",null,null],[6,"ErrorLED","","",null,null],[6,"StatusLED","","",null,null],[6,"CANRx","","",null,null],[6,"CANTx","","",null,null],[6,"SCL","","",null,null],[6,"SDA","","",null,null],[6,"USBDMinus","","",null,null],[6,"USBDPlus","","",null,null],[6,"Axis1","","",null,null],[6,"Axis2","","",null,null],[3,"CurrentDACChannel","sm4_firmware::prelude","",null,null],[12,"channel","","",0,null],[5,"initialize_current_ref","","",null,[[["currentref1pin",6],["dac",3],["currentref2pin",6]]]],[3,"GPIO","","",null,null],[12,"dir1","","",1,null],[12,"dir2","","",1,null],[12,"mode1","","",1,null],[12,"mode2","","",1,null],[12,"ref1","","",1,null],[12,"ref2","","",1,null],[12,"en1","","",1,null],[12,"en2","","",1,null],[12,"step1","","",1,null],[12,"step2","","",1,null],[12,"err1","","",1,null],[12,"err2","","",1,null],[12,"battery_voltage","","",1,null],[12,"error_led","","",1,null],[12,"status_led","","",1,null],[12,"can_rx","","",1,null],[12,"can_tx","","",1,null],[12,"scl","","",1,null],[12,"sda","","",1,null],[12,"usb_minus","","",1,null],[12,"usb_plus","","",1,null],[3,"LEDs","","",null,null],[12,"status_led","","",2,null],[12,"error_led","","",2,null],[3,"Monitoring","","",null,null],[12,"transfer","","",3,null],[12,"temperature","","",3,null],[12,"battery_voltage","","",3,null],[12,"transfer_ongoing","","",3,null],[3,"StepCounterEncoder","","",null,null],[12,"timer","","",5,null],[12,"past_position","","",5,null],[12,"current_position","","",5,null],[12,"current_velocity","","",5,null],[12,"direction","","",5,null],[12,"sampling_period","","",5,null],[12,"past_value","","",5,null],[3,"StepGeneratorTimer","","",null,null],[12,"timer","","",6,null],[12,"clocks","","",6,null],[12,"frequency","","",6,null],[3,"USBProtocol","","",null,null],[12,"serial","","",7,null],[12,"usb_dev","","",7,null],[3,"Storage","","",null,null],[12,"flash","","",9,null],[3,"ObjectDictionary","","The object dictionary struct represents the global state …",null,null],[12,"battery_voltage","","",17,null],[12,"temperature","","",17,null],[12,"axis1","","",17,null],[12,"axis2","","",17,null],[3,"DriverState","","",null,null],[12,"nmt_state","","",19,null],[12,"object_dictionary","","",19,null],[12,"last_received_speed_command_down_counter","","",19,null],[3,"SM4","sm4_firmware","",null,null],[12,"leds","","",18,null],[12,"usb","","",18,null],[12,"can","","",18,null],[12,"monitoring","","",18,null],[12,"state","","",18,null],[12,"axis1","","",18,null],[12,"axis2","","",18,null],[5,"panic","","",null,[[]]],[7,"COUNT","","",null,null],[11,"from","sm4_firmware::blocks::current_reference","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"into","","",0,[[]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","sm4_firmware::blocks::gpio","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"into","","",1,[[]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","sm4_firmware::blocks::leds","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"into","","",2,[[]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","sm4_firmware::blocks::monitoring","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"into","","",3,[[]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","sm4_firmware::blocks::step_counter","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"into","","",5,[[]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","sm4_firmware::blocks::step_timer","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"into","","",6,[[]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","sm4_firmware::blocks::usb","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"into","","",7,[[]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","sm4_firmware::blocks::eeprom","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"into","","",8,[[]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"into","","",9,[[]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"from","sm4_firmware::blocks::flash","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"into","","",10,[[]]],[11,"try_into","","",10,[[],["result",4]]],[11,"into_iter","","",10,[[]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"into","","",11,[[]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"from","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"into","","",13,[[]]],[11,"try_into","","",13,[[],["result",4]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"from","sm4_firmware::can","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"into","","",14,[[]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"from","","",15,[[]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"into","","",15,[[]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"from","sm4_firmware::object_dictionary","",17,[[]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"into","","",17,[[]]],[11,"try_into","","",17,[[],["result",4]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"from","sm4_firmware::sm4","",18,[[]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"into","","",18,[[]]],[11,"try_into","","",18,[[],["result",4]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"from","sm4_firmware::state","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"into","","",19,[[]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"fmt","sm4_firmware::blocks::flash","",11,[[["formatter",3]],["result",6]]],[11,"drop","","",13,[[]]],[11,"try_from","sm4_firmware::can","",15,[[["u16",15]],["result",4]]],[11,"next","sm4_firmware::blocks::flash","",10,[[],["option",4]]],[11,"clone","sm4_firmware::blocks::eeprom","",8,[[],["page",4]]],[11,"clone","sm4_firmware::blocks::flash","",11,[[],["error",4]]],[11,"clone","sm4_firmware::can","",15,[[],["canopenmessage",4]]],[11,"clone","sm4_firmware::object_dictionary","",17,[[],["objectdictionary",3]]],[11,"clone","sm4_firmware::state","",19,[[],["driverstate",3]]],[11,"get_velocity","sm4_firmware::blocks::step_counter","",5,[[],["velocity",3]]],[11,"get_position","","",5,[[],["position",3]]],[11,"reset_position","","",5,[[],["position",3]]],[11,"sample","","",5,[[]]],[11,"notify_direction_changed","","",5,[[["direction",4]]]],[11,"set_step_frequency","sm4_firmware::blocks::step_timer","",6,[[["hertz",3]]]],[11,"set_step_frequency","","",6,[[["hertz",3]]]],[11,"set_output_voltage","sm4_firmware::blocks::current_reference","",0,[[["u16",15]]]]],"p":[[3,"CurrentDACChannel"],[3,"GPIO"],[3,"LEDs"],[3,"Monitoring"],[8,"Counter"],[3,"StepCounterEncoder"],[3,"StepGeneratorTimer"],[3,"USBProtocol"],[4,"Page"],[3,"Storage"],[3,"MemIter"],[4,"Error"],[8,"FlashExt"],[3,"UnlockedFlash"],[3,"CANOpen"],[4,"CANOpenMessage"],[8,"CANOpenFrame"],[3,"ObjectDictionary"],[3,"SM4"],[3,"DriverState"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);