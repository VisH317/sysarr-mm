## Etched GPU simulator to test IR

**Parts of the sim:**
- infrastructure sim in Rust - SysArr, IPU, SAU, WCU, Memory Controllers (HBM), SRAM/Cache controllers
- kernels - designate different operations that need to be run and their parameters
- IR - create the operations to compile into

**Completed Parts so far**
- SysArr, WCU
