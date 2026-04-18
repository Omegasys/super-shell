# Design Notes

## 🧠 Philosophy

This project is not just another shell—it is an attempt to redefine how users
interact with systems through the command line.

We aim to balance:
- Power (Zsh)
- Stability (Bash)
- Usability (Fish)

---

## ⚖️ Key Tradeoffs

### 1. POSIX Compatibility vs Innovation
- POSIX ensures compatibility with existing scripts
- Innovation may require breaking compatibility

**Decision:**
Support POSIX-like mode + extended modern mode

---

### 2. Simplicity vs Power
- Simple shells are easy to use
- Powerful shells become complex quickly

**Decision:**
Layered complexity:
- Beginner-friendly defaults
- Advanced features opt-in

---

### 3. Performance vs Abstraction
- High abstraction can slow execution
- Low-level control increases complexity

**Decision:**
Keep execution close to system level, abstract only where needed

---

## 🧩 Core Design Concepts

### Command Representation
- Current: simple struct (`program + args`)
- Future: full AST with:
  - Pipelines
  - Redirection
  - Conditional execution

---

### Execution Model
- Current: sequential execution
- Future:
  - DAG-based execution
  - Parallel processing
  - Dependency resolution

---

### Builtins vs External Commands
- Builtins:
  - Faster
  - Integrated into shell state

- External:
  - Flexible
  - System-level execution

---

## 🔮 Experimental Ideas

- AI-assisted command generation
- Predictive execution
- Self-optimizing shell runtime
- Command intent recognition
- Secure capability-based execution

---

## 🛠️ Future Refactors

- Replace simple parser with tokenizer + AST builder
- Introduce plugin API
- Add configuration system (TOML or YAML)
- Refactor executor into pluggable backends

---

## 📌 Notes for Contributors

- Keep modules loosely coupled
- Avoid unnecessary dependencies
- Prioritize readability over cleverness
- Document all non-obvious logic

---

## 💡 Open Questions

- Should the shell be fully POSIX-compatible?
- Should plugins run in-process or sandboxed?
- How much AI integration is appropriate?
- What is the ideal balance between CLI and GUI?

---
