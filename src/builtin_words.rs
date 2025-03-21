- **Integer types:** `i32`, `i64`
- **Floating-point types:** `f32`, `f64`

**Examples:**
- Arithmetic: `i32.add`, `i64.mul`, `f32.div`
- Bitwise: `i32.and`, `i32.or`, `i32.xor`
- Comparisons: `i32.eq`, `i32.lt_s`, `f64.gt`

---

### **2. Reference Instructions**
Handle references for objects like functions or tables.

**Examples:**
- `ref.null` — Null reference
- `ref.func` — Reference to a function
- `ref.is_null` — Check if a reference is null

---

### **3. Parametric Instructions**
Operate on values of unspecified types.

**Examples:**
- `drop` — Pops the top value from the stack
- `select` — Chooses between two values based on a condition

---

### **4. Variable Instructions**
Access and manipulate local and global variables.

**Examples:**
- `local.get`, `local.set`, `local.tee` — Load, store, or duplicate local values
- `global.get`, `global.set` — Load and store global values

---

### **5. Table Instructions**
Operate on WebAssembly's table data structure (used for function pointers and dynamic dispatch).

**Examples:**
- `table.get`, `table.set` — Access table elements
- `table.size`, `table.grow` — Resize the table

---

### **6. Memory Instructions**
Access and manipulate linear memory (WebAssembly's primary memory model).

**Examples:**
- Load instructions: `i32.load`, `f64.load`
- Store instructions: `i32.store`, `f64.store`
- Memory size and growth: `memory.size`, `memory.grow`

---

### **7. Control Flow Instructions**
Used for branching, looping, and structured control flow.

**Examples:**
- `block`, `loop`, `if`, `else`
- `br` (branch), `br_if` (conditional branch)
- `return`, `unreachable`

---

### **8. SIMD Instructions (Optional Feature)**
For data-level parallelism, these operate on 128-bit vector types.

**Examples:**
- `v128.add`, `i8x16.add`, `f32x4.mul`

---

### **9. Atomic Instructions (Optional Feature)**
For concurrent memory operations in multi-threaded environments.

**Examples:**
- `i32.atomic.rmw.add`, `memory.atomic.wait`

---

### **Key Features of WebAssembly Instructions**
✅ **Compact Encoding:** Efficient binary format for fast decoding.  
✅ **Portable:** Same instruction set across all platforms.  
✅ **Secure:** No direct access to host machine resources; operates within a sandbox.

Would you like a deeper dive into any specific category or examples for clarity?