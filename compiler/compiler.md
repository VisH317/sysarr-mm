## The Compiler

**Basic Info**
- Starts with the torch.fx.graph
- Runs frontend optimizations on the graph
- Converts into IR instructions
- Runs IR-related optimizations for memory allocations, configuration on the IPU, etc.