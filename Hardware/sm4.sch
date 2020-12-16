EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L MCU_ST_STM32F4:STM32F412RETx U?
U 1 1 5FDA65D6
P 2200 2700
F 0 "U?" H 2200 811 50  0000 C CNN
F 1 "STM32F412RETx" H 2200 720 50  0000 C CNN
F 2 "Package_QFP:LQFP-64_10x10mm_P0.5mm" H 1600 1000 50  0001 R CNN
F 3 "http://www.st.com/st-web-ui/static/active/en/resource/technical/document/datasheet/DM00213872.pdf" H 2200 2700 50  0001 C CNN
	1    2200 2700
	1    0    0    -1  
$EndComp
$Comp
L Regulator_Switching:LM2594HVM-5.0 U?
U 1 1 5FDA8F15
P 7300 2050
F 0 "U?" H 7300 2417 50  0000 C CNN
F 1 "LM2594HVM-5.0" H 7300 2326 50  0000 C CNN
F 2 "Package_SO:SOIC-8_3.9x4.9mm_P1.27mm" H 7500 1800 50  0001 L CIN
F 3 "http://www.ti.com/lit/ds/symlink/lm2594.pdf" H 7300 2150 50  0001 C CNN
	1    7300 2050
	1    0    0    -1  
$EndComp
$Comp
L Regulator_Linear:MCP1700-3302E_SOT23 U?
U 1 1 5FDAAD88
P 9000 1900
F 0 "U?" H 9000 2142 50  0000 C CNN
F 1 "MCP1700-3302E_SOT23" H 9000 2051 50  0000 C CNN
F 2 "Package_TO_SOT_SMD:SOT-23" H 9000 2125 50  0001 C CNN
F 3 "http://ww1.microchip.com/downloads/en/DeviceDoc/20001826D.pdf" H 9000 1900 50  0001 C CNN
	1    9000 1900
	1    0    0    -1  
$EndComp
$Comp
L Driver_Motor:TMC2100-TA U?
U 1 1 5FDAFBA2
P 9550 4650
F 0 "U?" H 9550 3561 50  0000 C CNN
F 1 "TMC2100-TA" H 9550 3470 50  0000 C CNN
F 2 "Package_QFP:TQFP-48-1EP_7x7mm_P0.5mm_EP5x5mm_ThermalVias" H 9550 3550 50  0001 C CNN
F 3 "https://www.trinamic.com/fileadmin/assets/Products/ICs_Documents/TMC2100_datasheet_Rev1.08.pdf" H 8400 5700 50  0001 C CNN
	1    9550 4650
	1    0    0    -1  
$EndComp
$Comp
L Driver_Motor:TMC2100-TA U?
U 1 1 5FDB2CF3
P 7400 4650
F 0 "U?" H 7400 3561 50  0000 C CNN
F 1 "TMC2100-TA" H 7400 3470 50  0000 C CNN
F 2 "Package_QFP:TQFP-48-1EP_7x7mm_P0.5mm_EP5x5mm_ThermalVias" H 7400 3550 50  0001 C CNN
F 3 "https://www.trinamic.com/fileadmin/assets/Products/ICs_Documents/TMC2100_datasheet_Rev1.08.pdf" H 6250 5700 50  0001 C CNN
	1    7400 4650
	1    0    0    -1  
$EndComp
$EndSCHEMATC
