Here are 8 research questions to surface SOTA understanding for semantic code description generation:

---

### 1. Code Summarization Techniques
**"What are the current best approaches for automatic source code summarization and documentation generation?"**

Looking for: Neural approaches (CodeBERT, CodeT5, GPT-based), AST-based methods, hybrid approaches. What features matter most for generating accurate summaries?

---

### 2. Architectural Role Detection
**"How do automated architecture recovery tools classify source files into architectural roles (controller, service, repository, utility)?"**

Looking for: Heuristics used by tools like Structure101, Lattix, or academic tools. What signals (naming conventions, import patterns, class hierarchies) are most predictive?

---

### 3. Doc Comment Mining
**"What techniques exist for extracting and synthesizing documentation from source code comments, docstrings, and inline comments?"**

Looking for: NLP approaches for parsing doc comments, quality filtering, handling incomplete/stale comments, cross-referencing comments with code behavior.

---

### 4. Code Pattern Recognition for Intent
**"How can static analysis detect high-level code patterns like 'HTTP handler', 'database access', 'configuration loader', or 'event emitter'?"**

Looking for: AST pattern matching, API usage fingerprinting, call graph analysis. What libraries/frameworks have recognizable signatures?

---

### 5. Dependency-Based Role Inference
**"How do import/dependency graphs inform the architectural role or layer of a module in a codebase?"**

Looking for: Fan-in/fan-out metrics, dependency direction analysis, layering violation detection. How do tools like NDepend or SonarQube infer module importance?

---

### 6. LLM Code Understanding Benchmarks
**"What benchmarks and evaluation metrics exist for LLM-based code understanding, specifically for generating 'what does this file do' descriptions?"**

Looking for: CodeSearchNet, CodeXGLUE, HumanEval adjacents. How is semantic accuracy measured vs. syntactic similarity?

---

### 7. Semantic Clustering of Codebases
**"What techniques exist for automatically clustering or grouping source files by semantic purpose rather than directory structure?"**

Looking for: Topic modeling on code (LDA on identifiers), embedding-based clustering, feature extraction from AST. How do results compare to human-organized structures?

---

### 8. WHY vs WHAT in Code Documentation
**"What research exists on distinguishing 'what code does' from 'why code exists' in automated documentation, and how can rationale be inferred?"**

Looking for: Design rationale extraction, commit message mining, issue/PR linking, architectural decision records. This is the hardest part — the WHY is often not in the code itself.

---

### Bonus Context for Your Research

When compiling results, look for:
- **Rust-specific tooling** (rust-analyzer's semantic understanding, cargo-doc extraction)
- **Lightweight/fast approaches** (we need something that runs in a CLI, not a GPU cluster)
- **Heuristic baselines** (what simple rules get you 80% of the value?)
- **Failure modes** (when do automated descriptions go wrong?)

The sweet spot for semmap is probably: **doc comment extraction + pattern-based classification + dependency fan-in/out analysis** — all achievable without an LLM at runtime.


***

For automatic source code summarization and documentation without using Large Language Models (LLMs), the most effective approaches rely on **Static Analysis**, **Information Retrieval (IR)**, and **Template-Based Generation**.

These "programmatic" techniques are deterministic, meaning they produce consistent outputs based on the code's structure, syntax, and keywords rather than probabilistic generation.

### 1. Template-Based Summarization (The Standard Approach)
This is the most widely used industrial approach. It uses predefined sentence structures and fills in the blanks with information extracted from the code's Abstract Syntax Tree (AST).

*   **How it works:** You define templates for different types of methods (e.g., "Returns the [Return Type] of the [Object]"). A parser extracts the method signature, return type, and parameters to populate the template.
*   **Best for:** API documentation, getters/setters, and boilerplate code.
*   **Tools:**
    *   **Doxygen / Javadoc / Sphinx:** While known for manual comments, these tools can be configured (often via plugins or scripts) to auto-generate skeleton documentation by parsing method signatures.
    *   **Custom AST Parsers:** Using libraries like `javaparser` (Java), `Roslyn` (C#), or the `ast` module (Python) to write scripts that traverse the code tree and output formatted strings.

### 2. Stereotype Identification (Heuristic-Based)
This technique classifies methods and classes into specific "stereotypes" or roles based on their structure and data flow, then assigns a summary based on that role.

*   **How it works:** The system analyzes the method's control flow and state changes to categorize it.
    *   *Example:* If a method accepts a value and assigns it to a private field, it is tagged as a **"Setter"**.
    *   *Example:* If a method returns a boolean based on a condition, it is a **"Predicate"**.
    *   *Example:* If a method creates an object, it is a **"Factory"**.
*   **Generation:** Once a method is classified (e.g., as a "Collaborator" or "Controller"), a specific summary template for that stereotype is used.
*   **Key Tool:** **StereoCode** (and similar academic tools based on `srcML`) is a notable example that annotates C++, C#, and Java code with these stereotypes programmatically.

### 3. Information Retrieval (IR) & Clone Detection
This approach assumes that "new" code is often similar to "old" code that has already been documented. It treats code as text documents.

*   **How it works:**
    1.  **Vector Space Model (VSM):** Code is converted into vectors based on term frequency (TF-IDF). Identifiers (variable names) and keywords are treated as "words."
    2.  **Retrieval:** When you need to summarize a new function, the system searches a database of existing, well-documented code for the most similar snippet (using cosine similarity).
    3.  **Reuse:** The documentation from the retrieved "clone" or similar snippet is adapted or suggested as the summary for the new code.
*   **Best for:** Large legacy codebases where developers frequently copy-paste or reuse patterns.

### 4. SWUM (Software Word Usage Model)
SWUM is a linguistic approach that focuses on extracting semantics from variable names and function identifiers without using AI training data.

*   **How it works:**
    1.  **Tokenization:** It splits complex identifiers (e.g., `getUserProfileFromDB`) into constituent words (`get`, `User`, `Profile`, `From`, `DB`).
    2.  **Part-of-Speech Tagging:** It applies grammatical rules to understand that `get` is a verb and `UserProfile` is the object.
    3.  **Sentence Construction:** It rearranges these tokens into a natural sentence: *"Gets the user profile from the DB."*
*   **Why it works:** Developers tend to name functions after the action they perform. SWUM formalizes the extraction of this intent programmatically.

### 5. AST Flattening & Autofolding
Instead of generating new text, this approach "summarizes" code by intelligently hiding low-value lines to show only the critical logic.

*   **How it works:**
    *   The algorithm builds a Control Flow Graph (CFG) or AST.
    *   It calculates the "cyclomatic complexity" or "centrality" of different nodes.
    *   It folds (hides) simple error handling, logging, or boilerplate blocks.
    *   It keeps the "backbone" of the logic visible.
*   **Result:** The "summary" is a condensed version of the actual code (a snippet) rather than an English paragraph.

### Summary of Recommendations

| Goal | Best Programmatic Approach |
| :--- | :--- |
| **High Precision API Docs** | **Template-Based** (using AST parsing) |
| **Understanding Logic** | **Stereotype Identification** (Classify logic type) |
| **Leveraging Legacy Code** | **Information Retrieval** (Find & reuse similar docs) |
| **Readable Identifiers** | **SWUM** (Linguistic expansion of function names) |

**For a strict programmatic implementation:** I would recommend building a pipeline that first runs **Stereotype Identification** to classify the method. If it falls into a known category (Setter, Getter, Factory), use a **Template**. If it is a complex algorithmic method, fall back to **SWUM** rules to convert the function name `verb-noun` pairs into a readable sentence.

Automated architecture recovery tools classify source files into architectural roles (Controller, Service, Repository, Utility) using a combination of **lexical rules (naming)**, **structural analysis (dependencies)**, and **semantic stereotypes (code signatures)**.

While commercial tools like **Structure101** and **Lattix** primarily rely on dependency structure and user-defined naming patterns, academic tools (like **Stereocode** or **ARC**) often employ deeper static analysis to detect "code stereotypes."

### 1. Classification Heuristics by Role

Tools classify files by looking for specific "signals" that distinguish one role from another.

| Architectural Role | **Primary Signal (Naming/Lexical)** | **Structural Signal (Dependencies)** | **Semantic Signal (Code Signature)** |
| :--- | :--- | :--- | :--- |
| **Controller** | Suffix: `*Controller`, `*Resource`, `*Action` | **Incoming:** From Web/Framework<br>**Outgoing:** To `*Service`, `*Manager` | Methods mapped to HTTP verbs (`GET`, `POST`). Annotations: `@RestController`, `@RequestMapping`. |
| **Service** | Suffix: `*Service`, `*Manager`, `*Logic` | **Incoming:** From `*Controller`<br>**Outgoing:** To `*Repository`, `*DAO`, `*Model` | High Cyclomatic Complexity (contains business logic). Methods often wrap transactions (`@Transactional`). |
| **Repository** | Suffix: `*Repository`, `*DAO` | **Incoming:** From `*Service`<br>**Outgoing:** To Database Drivers (JDBC/Hibernate) | Inherits from specific base classes (`JpaRepository`). Methods are purely CRUD (create, read, update, delete). |
| **Utility** | Suffix: `*Util`, `*Helper`, `*Commons` | **Incoming:** High Fan-in (used by everyone)<br>**Outgoing:** Zero/Low (only depends on JDK/System) | **Stateless:** All methods are `static`. No private fields. High cohesiveness but low coupling to domain objects. |
| **Entity / Model** | Suffix: `*Model`, `*DTO`, or no suffix | **Incoming:** High Fan-in (passed as data)<br>**Outgoing:** Minimal (depends mostly on JDK types) | **Stateful:** Mostly private fields + Getters/Setters. Annotations: `@Entity`, `@Table`. |

---

### 2. Detection Techniques Used by Tools

#### A. Commercial Tools (Structure101, Lattix)
These tools focus on **"conformance"** and **"levelization"** rather than "blind discovery." They rarely guess a role; they assign it based on rules you configure or patterns they detect.

*   **Pattern Matching (Auto-mapping):**
    *   **Mechanism:** They use regex patterns to map class names to logical "cells" in the architecture.
    *   **Example:** A rule might state `**/service/**` maps to the "Service Layer."
*   **Levelization (Dependency Analysis):**
    *   **Mechanism:** They arrange classes into levels based on dependency direction. "Core" components (Utilities, Domain Entities) settle at the bottom because they don't depend on anything. "Controllers" float to the top because they depend on everything but are rarely called by other internal code (except the framework).
    *   **Tangle Detection:** If a "Repository" calls a "Controller," these tools flag it as a "cyclic dependency" or "tangle," signaling a violation of the architectural role.

#### B. Academic & Advanced Tools (Stereocode, JDeodorant, Dragan et al.)
These tools use **"Code Stereotyping"**—a technique to classify classes based on their internal behavior rather than just their name.

*   **Method Stereotypes:**
    *   First, they classify methods into types like *Accessor* (getter), *Mutator* (setter), *Creational* (constructor), or *Collaborator* (calls other objects).
*   **Class Stereotypes:**
    *   They aggregate method types to label the class.
    *   **Entity:** Mostly *Accessors* and *Mutators* (holds data).
    *   **Boundary (Controller):** mostly *Collaborators* that interact with external inputs (IO/UI).
    *   **Control (Service):** High complexity *Commander* methods that orchestrate logic between other classes.

---

### 3. Key Predictive Signals

1.  **Naming Conventions (The 80/20 Rule):**
    *   Most tools rely 80% on naming. If a class is named `UserRepo`, tools immediately assume it belongs to the Persistence layer.
    *   *Tools:* All (SonarQube, Structure101, Lattix).

2.  **Import Patterns:**
    *   **Controller:** Imports `org.springframework.web.*`, `javax.servlet.*`.
    *   **Repository:** Imports `java.sql.*`, `javax.persistence.*`, `org.hibernate.*`.
    *   **Utility:** Imports `java.util.*`, `java.lang.*` but almost no project-specific domain packages.

3.  **Inheritance Hierarchy:**
    *   Classes extending `HttpServlet` are automatically classified as **Controllers**.
    *   Classes implementing `JpaRepository` or extending `HibernateDaoSupport` are **Repositories**.

4.  **Fan-in / Fan-out (centrality):**
    *   **Utilities** have **High Fan-in** (many classes use them) and **Low Fan-out** (they use few other classes).
    *   **Services** have **Medium Fan-in** (called by Controllers) and **Medium Fan-out** (call Repositories).
    *   **Controllers** have **Low Fan-in** (nobody calls them directly) and **High Fan-out** (they call Services).


    Existing techniques for extracting and synthesizing documentation from source code can be categorized into **static analysis (parsing)**, **runtime introspection**, **AI-driven synthesis**, and **"inverse" literate programming**.

### 1. Static Analysis & Parsing (Traditional Extraction)
These techniques treat source code as text or a structured tree. They are deterministic and rely on strict formatting standards.

*   **Abstract Syntax Tree (AST) Parsing:**
    *   **Technique:** Tools parse the source code into an Abstract Syntax Tree (AST) to understand the grammatical structure (e.g., "this is a function," "this is a class"). They then associate specific comment blocks with the AST nodes they precede.
    *   **Synthesis Capability:** Low. It primarily organizes extracted text hierarchically (e.g., Class $\to$ Method $\to$ Parameter) but does not rewrite or summarize it.
    *   **Tools:** **Doxygen**, **Javadoc**, **Sphinx** (for Python), **ESDoc** (JavaScript).

*   **Docstring Parsing:**
    *   **Technique:** Relies on language-specific conventions for documentation blocks (e.g., `"""` in Python, `/**` in Java/C++). Parsers extract these specific strings and ignore other comments.
    *   **Synthesis Capability:** Extracts structure but relies on the developer to write the "synthesis" inside the docstring.
    *   **Tools:** **Pydoc**, **GoDoc**, **Rustdoc**.

*   **Annotation/Decorator Processing:**
    *   **Technique:** Uses metadata (annotations in Java, decorators in Python/TS) rather than natural language comments to define documentation. This is common for API documentation where behavior (like HTTP methods or constraints) is more important than narrative.
    *   **Synthesis Capability:** Synthesizes a structured specification (like a JSON/YAML file) from scattered code attributes.
    *   **Tools:** **Swagger/OpenAPI** (via annotations), **FastAPI** (Python).

### 2. AI & NLP-Driven Synthesis (Modern Generation)
These techniques use Natural Language Processing (NLP) and Large Language Models (LLMs) to *understand* the code and comments, allowing them to generate new text (synthesis) rather than just copying it.

*   **LLM-Based Summarization:**
    *   **Technique:** Feeds code snippets + inline comments into an LLM (like GPT-4, Claude, or StarCoder). The model is prompted to "explain this function" or "generate a README."
    *   **Synthesis Capability:** High. It can read cryptic inline comments (e.g., `// hack fix for bug #33`) and synthesize them into a professional description (e.g., "Handles edge case related to issue #33").
    *   **Tools:** **GitHub Copilot**, **Ghostwriter**, **Mintlify**, **Tabnine**.

*   **Retrieval-Augmented Generation (RAG):**
    *   **Technique:** Indexes the entire codebase (comments and logic). When a user asks "How does authentication work?", the system retrieves relevant comment blocks and code, then synthesizes a specific answer.
    *   **Synthesis Capability:** Creates on-demand, context-aware documentation that didn't exist in a static file.

*   **Semantic Clustering:**
    *   **Technique:** Uses embedding models (like CodeBERT) to vectorise code and comments. Similar functions are clustered together to generate high-level "topic" documentation, helping describe *modules* rather than just *functions*.

### 3. Runtime Introspection (Dynamic Extraction)
These techniques extract documentation from the program *while it is running*.

*   **Reflection/Introspection:**
    *   **Technique:** Languages like Python and Lisp treat documentation as a first-class object. You can access a function's documentation programmatically at runtime (e.g., `my_func.__doc__` in Python).
    *   **Synthesis Capability:** Allows for self-documenting CLI tools (e.g., a `--help` command that prints the docstrings of the functions it calls).
    *   **Tools:** Python's `inspect` module, Java Reflection API.

### 4. "Inverse" Literate Programming
Literate programming is writing a book that contains code. "Inverse" literate programming is taking a code file and turning it into a book-like document.

*   **Side-by-Side Generation:**
    *   **Technique:** The tool splits the file into "code" and "comment" segments. It produces a two-column HTML layout: narrative on the left (derived from comments) and code on the right.
    *   **Synthesis Capability:** Creates a narrative flow from inline comments, making the code read like a tutorial.
    *   **Tools:** **Docco** (JavaScript), **Pycco** (Python), **Rocco** (Ruby).

### Summary of Techniques

| Technique | Best For | Code "Understanding" | Example Tools |
| :--- | :--- | :--- | :--- |
| **AST Parsing** | API Reference, Class Hierarchies | Structural only | Doxygen, Javadoc |
| **LLM Synthesis** | Explaining logic, Summarizing | Semantic (Deep) | Copilot, Mintlify |
| **Annotations** | REST APIs, Schemas | Metadata-based | Swagger/OpenAPI |
| **Introspection** | CLI Help, REPLs | Runtime access | Python `inspect` |
| **Inverse Literate** | Tutorials, Walkthroughs | Linear narrative | Docco, Pycco |

Static analysis detects high-level patterns by moving beyond simple text matching (like `grep`) to analyze the code's **structure** (Abstract Syntax Tree or AST) and **behavior** (Control Flow and Data Flow).

Tools like **CodeQL**, **Semgrep**, and **SonarQube** identify these patterns by defining "rules" that look for specific combinations of function calls, class inheritance, annotations, and variable usage.

Here is how static analysis detects each of the specific patterns you requested:

### 1. HTTP Handler
Static analysis tools identify HTTP handlers by looking for framework-specific "entry points." Since web frameworks (like Spring, Express, Django) rely on conventions, the analysis tools allow you to model those conventions.

*   **Mechanism:**
    *   **Annotation/Decorator Matching:** In languages like Java (Spring) or Python (Django/Flask), tools look for specific decorators. For example, a rule might define an "HTTP Handler" as *any class method annotated with `@RequestMapping`, `@GetMapping`, or `@post`*.
    *   **Method Signature Analysis:** In framework-agnostic contexts, tools might look for functions that take specific parameters (e.g., `(req, res)` in JavaScript) or inherit from a specific base class (e.g., `HttpServlet` in Java).
    *   **Call-Graph Entry Points:** Security tools mark these handlers as "Sources"—places where untrusted user input enters the application.

*   **Concrete Example (CodeQL/Java):**
    CodeQL has a built-in library class `SpringControllerMethod`. It identifies any method inside a class annotated with `@Controller` or `@RestController`.
    ```ql
    import java
    import semmle.code.java.frameworks.spring.Spring

    from SpringControllerMethod m
    select m, "This is an HTTP handler."
    ```

### 2. Database Access
To detect database access, static analysis looks for "Sinks"—functions known to execute queries or interact with storage.

*   **Mechanism:**
    *   **API/Library Recognition:** Tools maintain a list of known database libraries (e.g., `java.sql.Statement`, `psycopg2`, `Sequelize`, `Hibernate`).
    *   **String Analysis (for Raw SQL):** Tools look for string literals containing keywords like `SELECT`, `INSERT`, or `UPDATE` that are passed to specific functions.
    *   **ORM Method Calls:** For Object-Relational Mappers (ORMs), the analysis looks for calls to methods like `.find()`, `.save()`, or `.query()`.

*   **Concrete Example (Semgrep):**
    This rule looks for any call to `execute` where the string starts with common SQL commands.
    ```yaml
    rules:
      - id: detect-sql-execution
        patterns:
          - pattern: $DB.execute("..." + ...)
          - pattern-regex: "(?i)(SELECT|INSERT|UPDATE|DELETE).*"
        message: "Database access detected via raw SQL."
    ```

### 3. Configuration Loader
Detecting configuration loading involves identifying code that reads from the environment, specific file formats, or external services.

*   **Mechanism:**
    *   **Environment Variable Access:** Tools look for calls to standard library functions like `os.getenv()` (Python), `process.env` (Node.js), or `System.getenv()` (Java).
    *   **File I/O with Specific Extensions:** Analysis rules can flag file read operations where the filename ends in `.json`, `.yaml`, `.xml`, or `.env`.
    *   **Key-Value Usage:** Heuristics often look for variables named `config`, `settings`, or `secrets` being assigned the result of a file read operation.

*   **Concrete Example (General Logic):**
    A static analysis rule might look for the following sequence in the Data Flow Graph:
    1.  Function call to `fs.readFile` or `dotenv.config`.
    2.  The resulting variable is named `config` OR the file path argument contains `.env`.

### 4. Event Emitter
This pattern is detected by identifying the "Observer" design pattern or specific library usage (like Node.js `EventEmitter` or C# `delegate`s).

*   **Mechanism:**
    *   **Registration & Triggering:** Tools look for pairs of calls: one that registers a listener (e.g., `.on()`, `.addListener()`, `+= handler`) and one that triggers it (e.g., `.emit()`, `.invoke()`).
    *   **Class Inheritance:** In Node.js, tools check if a class `extends EventEmitter`. In Java, they might look for classes implementing `Observer` or defining `Listener` interfaces.
    *   **Callback Analysis:** Static analysis verifies that the function passed to `.on("event", fn)` matches the signature expected by the `.emit("event", data)` call elsewhere in the codebase.

*   **Concrete Example (JavaScript/Node.js):**
    Static analysis can map the "event name" string to link the emitter and the listener.
    *   **Source:** `myEmitter.emit('userLoggedIn', user);`
    *   **Sink:** `myEmitter.on('userLoggedIn', (user) => { ... });`
    *   **Analysis:** The tool matches the string literal `'userLoggedIn'` to determine that these two separate lines of code are part of the same logical "Event Emitter" pattern.

### Summary Table

| Pattern | Key Static Analysis Signal | Example Detection Logic |
| :--- | :--- | :--- |
| **HTTP Handler** | Annotations, Decorators, Inheritance | Class has `@Controller` OR method calls `app.get('/route', ...)` |
| **DB Access** | Library imports, SQL keywords | Call to `executeQuery()` OR string contains "SELECT * FROM" |
| **Config Loader** | Env vars, File extensions | Call to `getenv()` OR reading file ending in `.env`/`.yaml` |
| **Event Emitter** | "On/Emit" pairs, Interfaces | Call to `.emit('name')` matches `.on('name')` |

Import and dependency graphs are the "X-rays" of a codebase. While folder structures represent the **intended** organization, dependency graphs reveal the **actual** architectural reality.

By analyzing the direction of arrows (who imports whom), the density of connections (fan-in/fan-out), and the nature of external dependencies, you can objectively determine the architectural role of a module.

Here is how dependency graphs inform the layer and role of a module:

---

### 1. Fan-In vs. Fan-Out (Stability vs. Volatility)
The most immediate indicator of a module's role is the ratio of incoming imports (Fan-in) to outgoing imports (Fan-out).

*   **High Fan-In, Low Fan-Out (The "Sink")**
    *   **Role:** Utilities, Shared Kernels, Data Entities, or Core Domain types.
    *   **Layer:** Bottom-most layer (Infrastructure-agnostic) or Center (in Clean Architecture).
    *   **Interpretation:** These modules are **Stable**. Changing them is expensive because it breaks many other parts of the system. Therefore, they usually contain fundamental building blocks (e.g., `StringHelpers`, `User`, `Money`).
*   **Low Fan-In, High Fan-Out (The "Source")**
    *   **Role:** Entry points, Scripts, Orchestrators, or Main Controllers.
    *   **Layer:** Top-most layer (Presentation or Composition Root).
    *   **Interpretation:** These modules are **Volatile**. They tie many things together to execute a specific workflow. They are easy to change because nothing depends on them.
*   **High Fan-In, High Fan-Out (The "Hub")**
    *   **Role:** "God Objects" or central mediators.
    *   **Interpretation:** **Architectural Warning.** This usually indicates a module that knows too much and is relied upon by too many people. Ideally, layers should move from unstable to stable; a hub contradicts this by being coupled to everything while everyone is coupled to it.

### 2. The Direction of Dependencies (Layering)
In almost all sound architectural patterns (Layered, Onion, Hexagonal), the "Dependency Rule" dictates that **dependencies must point toward stability.**

*   **Strict Layering:** If Module A imports Module B, and Module B imports Module C, A is a higher layer (e.g., UI) and C is a lower layer (e.g., Database/Utility).
*   **Dependency Inversion (DIP):** In modern architectures, the graph reveals if you have successfully decoupled logic from infrastructure.
    *   *Without DIP:* Logic imports Database. (Graph: Logic $\to$ DB)
    *   *With DIP:* Database imports Logic (via interfaces). (Graph: DB $\to$ Logic)
    *   **Conclusion:** If you see a module named "BusinessRules" importing "SQLAlchemy" or "Axios," the graph proves the architectural layer has been breached (Leaky Abstraction).

### 3. Proximity to External Boundaries (The "Islands")
You can determine a module's role by looking at what *third-party* libraries it imports.

*   **The UI Layer:** Imports frameworks like `React`, `SwiftUI`, or `DOM` manipulation libraries. These should exist at the edge of the graph.
*   **The Infrastructure Layer:** Imports IO-heavy libraries like `fs` (file system), `pg` (Postgres driver), `aws-sdk`, or `http` clients.
*   **The Pure Domain Layer:** Imports **nothing** (or only standard language libraries and other internal pure modules).
    *   *Visual check:* If you isolate a module in the graph and it has zero arrows pointing to external frameworks, it is likely the **Core Domain**.

### 4. Cycles and Strongly Connected Components
When the graph shows a cycle (A $\to$ B $\to$ A), it informs you that these modules are effectively **one single architectural unit**.

*   **Role:** If a "View" imports a "Controller," and the "Controller" imports the "View," they are inseparable. They cannot act as independent layers.
*   **Architectural Implication:** Cycles blur layer boundaries. If a cycle spans across what you thought were two different layers, those layers no longer exist; they have collapsed into a "Big Ball of Mud."

### 5. Transitive Impact (The Blast Radius)
The graph shows the "blast radius" of a change, which helps define the module's architectural criticality.

*   **Critical Infrastructure:** If a module is 5 hops deep in the graph but acts as a dependency for 90% of the codebase (e.g., a logging wrapper or a base configuration module), it is structurally "foundational."
*   **Feature Silos:** If a cluster of modules interact heavily with each other but share only one thin connection to the rest of the graph, they represent a **Vertical Slice** or a standalone feature/plugin.

### Summary Table

| Pattern in Graph | Architectural Role | Typical Examples |
| :--- | :--- | :--- |
| **Depends on Nothing; Used by Many** | Stable Foundation | Models, Utils, Interfaces, Domain Entities |
| **Depends on Many; Used by None** | Volatile Orchestrator | `main.ts`, Dependency Injection Containers, CLI entry points |
| **Depends on UI Libs** | Presentation | React Components, ViewControllers |
| **Depends on IO/DB Libs** | Infrastructure | Repositories, API Clients, Adapters |
| **Depends on Domain; Used by UI** | Application Logic | Use Cases, Services, Interactors |
| **Circular Dependencies** | Coupled Component | Tightly coupled Views/Logic (Anti-pattern) |

Standardized benchmarks and metrics for "what does this file do" (file-level) code understanding are an emerging area, distinct from the more common function-level tasks. While function-level benchmarks (like CodeXGLUE) are well-established, file-level summarization often requires adapting generation benchmarks or using newer, specialized datasets.

### **1. Benchmarks for File-Level Understanding**

Most standard benchmarks focus on small units (functions). For **file-level** descriptions, researchers often adapt class-level or repository-level benchmarks.

| Benchmark | Level | Description | Relevance to "What does this file do" |
| :--- | :--- | :--- | :--- |
| **CodeXGLUE (Code-Text)** | Function | The industry standard, containing the **CodeSearchNet** dataset. | **Foundational.** While mostly function-level, it sets the baseline for summarization tasks. Some subsets can be aggregated to simulate file-level context. |
| **ClassEval** | Class/File | Originally for class-level code generation (Text-to-Code). | **High.** Researchers have inverted this benchmark to evaluate summarization (Code-to-Text). Since many files contain a single class, this is the closest proxy to "file-level" understanding. |
| **RepoBench** | Repository | Focuses on multi-file context for auto-completion (Retrieval, Completion, Pipeline). | **Contextual.** Useful for evaluating if a model understands *dependencies* (e.g., imports across files), which is critical for explaining what a file does in a larger project. |
| **Swe-bench** | Repository | Real-world GitHub issues and PRs. | **Advanced.** While a coding benchmark, it implicitly tests file understanding. To fix a bug, the model must first understand "what the file does." Often used to test "agentic" understanding. |
| **TL-CodeSum** | Function | Large-scale dataset of Java methods and Javadoc. | **Data Source.** Often used as training data, but less effective for file-level evaluation due to its granular focus. |

> **Emerging Research:** A recent key paper, *"Code Summarization Beyond Function Level"* (2024/2025), specifically critiques the lack of file-level benchmarks and introduces **"Modified ClassEval"** and repository-level datasets to test this exact capability.

---

### **2. Evaluation Metrics**

Evaluating "what does this file do" descriptions is harder than code generation because a "correct" summary can take many forms.

#### **A. Semantic & Alignment Metrics (Recommended)**
These are currently considered the most reliable for descriptive tasks because they measure meaning rather than just word overlap.

*   **SIDE (Summary alIgnment to coDe sEmantics):**
    *   **What it is:** A trained metric specifically designed to measure how well a summary aligns with the code's actual logic, independent of a reference summary.
    *   **Why use it:** It helps detect if a summary is "hallucinating" features that don't exist in the code, a common failure mode in LLM descriptions.
*   **LLM-as-a-Judge:**
    *   **What it is:** Using a strong model (e.g., GPT-4o) to grade a generated summary against a rubric.
    *   **Rubric Criteria:** Common criteria include **Completeness** (does it cover all main classes/functions?), **Conciseness** (is it free of fluff?), and **Factual Correctness** (does it accurately describe the logic?).
    *   **Pros/Cons:** correlates best with human judgment but suffers from "verbosity bias" (preferring longer, fluffier answers).

#### **B. Structure-Aware Metrics**
*   **CodeBLEU:**
    *   **What it is:** A variant of BLEU that injects code syntax (AST) and data-flow semantics.
    *   **Usage:** While primarily for code generation, it is increasingly used for summarization to ensure the description uses the correct variable names and identifies key data flows mentioned in the code.

#### **C. Traditional N-Gram Metrics (Legacy)**
You will see these in papers, but they are widely considered insufficient for "understanding" tasks.
*   **BLEU / ROUGE-L:** Measure word overlap.
*   **Limitations:** A summary can be perfect but get a low BLEU score if it uses synonyms (e.g., "iterates through the list" vs. "loops over the array"). They are **not recommended** as primary metrics for file descriptions.

### **Summary of Recommendation**
If you are building an evaluation pipeline for "what does this file do":
1.  **Dataset:** Use **ClassEval** (adapted for summarization) or create a "Gold Set" of ~50 file-summary pairs from your own repositories.
2.  **Primary Metric:** **LLM-as-a-Judge** (GPT-4) with a specific rubric (Accuracy, Coverage, Conciseness).
3.  **Secondary Metric:** **SIDE** or **CodeBERTScore** to check semantic alignment without relying solely on the LLM judge.

Techniques for clustering source files by semantic purpose (rather than directory structure) generally fall into three categories: **Vector-Space Clustering** (using code embeddings), **Graph-Based Community Detection** (analyzing dependency networks), and **LLM-Agentic Organization** (using AI reasoning).

The most robust technique is a **Hybrid Approach**, which combines the "what the code says" (semantics) with "how the code connects" (structure).

### 1. Vector-Space Clustering (Pure Semantic)
This technique treats code files like text documents, converting them into mathematical vectors (embeddings) and grouping them based on similarity.

*   **Code Embeddings:** Instead of standard text embeddings, use models trained specifically on code (e.g., **CodeBERT**, **CodeT5**, or **StarCoder**). These models understand syntax, variable naming, and comments.
*   **Dimensionality Reduction:** High-dimensional vectors (e.g., 768 dimensions) are often noisy. Techniques like **UMAP** (Uniform Manifold Approximation and Projection) or **t-SNE** reduce these to 2D or 3D space to reveal visual clusters.
*   **Clustering Algorithms:**
    *   **K-Means:** Good if you know the target number of modules/groups.
    *   **DBSCAN / HDBSCAN:** Better for codebases because they don't require specifying the number of clusters and can identify "noise" (utility files that don't fit anywhere).
    *   **Topic Modeling (LDA/BERTopic):** extract "topics" from code comments and identifiers to group files by concepts like "authentication," "payment processing," or "UI rendering."

### 2. Graph-Based Community Detection (Structural + Semantic)
This is often considered the state-of-the-art for **Software Architecture Recovery**. It views your codebase as a network where files are nodes and dependencies (imports, function calls) are edges.

*   **Weighted Graphs:** Construct a dependency graph (using tools like `networkx`) where the connection strength (weight) between two files is determined by their **semantic similarity**.
    *   *Example:* `File A` imports `File B`. If their semantic embedding similarity is high (0.85), the edge is strong. If low (0.1), it's weak.
*   **Community Detection Algorithms:**
    *   **Louvain or Leiden Algorithms:** These optimize "modularity," finding groups of nodes that are densely connected to each other but sparsely connected to the rest of the graph.
    *   **CESNA (Communities from Edge Structure and Node Attributes):** A specific algorithm designed to find overlapping communities by modeling the interaction between the graph structure and node attributes (semantic keywords).
*   **Spectral Clustering:** Uses the eigenvalues of the graph's Laplacian matrix to partition the code into groups, effectively "cutting" the graph at its weakest semantic links.

### 3. LLM-Agentic Organization
Instead of mathematical clustering, this approach uses Large Language Models to "reason" about file purpose.

*   **"Design-First" Deductive Recovery:** An LLM scans the codebase to generate a high-level `ARCHITECTURE.md` or design doc, then assigns every file to one of the components defined in that doc.
*   **Chain-of-Thought Organization:** Agents (using tools like `LangChain` or `Autogen`) iteratively read files and propose moves.
    *   *Workflow:* Agent reads file -> Summarizes purpose -> Checks existing clusters -> Decides to "Add to Auth Module" or "Create New Module."

### Practical Implementation Tools
While there is no single "magic CLI" for this yet, you can build a pipeline using these open-source libraries:

| Component | Recommended Tool/Library | Purpose |
| :--- | :--- | :--- |
| **Embeddings** | `sentence-transformers` | Generate vectors for code using `huggingface/CodeBERTa` or similar. |
| **Graphing** | `networkx` | Build dependency graphs (nodes=files, edges=imports). |
| **Clustering** | `scikit-learn` / `cdlib` | Run DBSCAN or Louvain/Leiden algorithms. |
| **Vector DB** | `FAISS` / `ChromaDB` | Store and query embeddings for large repos. |
| **Desktop Tool** | **AI File Sorter** | Open-source tool for organizing local files via local LLMs (mostly file-content based). |
| **Visualization** | `Cytoscape` / `Gephi` | Visually inspect the resulting clusters. |

#### Example: The "Hybrid" Script Recipe
A powerful Python script to do this would follow these steps:
1.  **Parse Imports:** Use Python's `ast` module (or `tree-sitter` for other languages) to build a directed graph of imports.
2.  **Generate Embeddings:** Run every file's source code through `sentence-transformers` to get a vector.
3.  **Weight Edges:** For every edge (import) in the graph, set `weight = cosine_similarity(vector_A, vector_B)`.
4.  **Detect Communities:** Run `community.best_partition(graph)` (Louvain algorithm).
5.  **Output:** Print the resulting groups. Files in the same "community" semantically and structurally belong together.

Research into automated documentation explicitly distinguishes between **"what code does"** (functional summarization) and **"why code exists"** (design rationale or intent). The consensus is that while standard tools excel at the former, the latter is significantly harder because the "why" often resides outside the code syntax itself.

The following breakdown details the key research areas and methods for inferring rationale.

### 1. The Distinction: Summarization vs. Rationale
*   **"What" (Summarization):** This describes the immediate behavior of the code (e.g., "Iterates through the list and removes duplicates"). Research typically uses Neural Machine Translation (NMT) approaches (like Code2Seq or CodeBERT) to map code tokens to natural language. These models often fail to capture intent, resulting in comments that merely restate the code.
*   **"Why" (Rationale):** This explains the purpose, business logic, or design choice (e.g., "Removes duplicates to ensure unique user IDs before database insertion"). Research indicates this requires understanding the **context**—how the code fits into the larger system or the historical reasons for its creation.

### 2. How Rationale is Inferred (Methodologies)
Since the "why" is rarely explicit in the code's Abstract Syntax Tree (AST), researchers use "multi-modal" sources to infer it.

#### A. Mining Commit Messages & History
*   **The Concept:** Commit messages are the richest source of rationale. Developers often write *what* they changed in the code, but explain *why* in the commit log.
*   **Technique:** Research (e.g., *Using Text Mining Techniques to Extract Rationale*) involves training classifiers to identify "rationale sentences" in commit logs—looking for keywords like "because," "due to," "fix," or "to allow."
*   **Application:** When documenting a legacy function, tools can retrieve the commit message associated with its last major change and summarize that into the documentation as the "intent."

#### B. Context-Aware Summarization
*   **The Concept:** A function's purpose is often defined by who calls it, not just what it contains.
*   **Technique:** Newer approaches (e.g., *Context-Aware Code Summary Generation*) feed the model not just the target function, but also its **call graph** (caller/callee relationships) and **file-level dependencies**.
*   **Example:** If a generic string-parsing function is called exclusively by a `SecuritySanitizer` class, the inferred rationale shifts from "Splits strings by comma" (what) to "Sanitizes input for security" (why).

#### C. Linking to Issue Trackers (Traceability)
*   **The Concept:** The "why" usually originates in a feature request or bug report (Jira, GitHub Issues).
*   **Technique:** Researchers use Information Retrieval (IR) to link code segments to specific issue tickets. The title and description of the linked issue are then used to seed the documentation generation.
*   **Key Term:** **"Software Traceability"**—automatically recovering the link between high-level requirements (Why) and low-level code (What).

#### D. Heuristic & Keyword Analysis
*   **The Concept:** Certain linguistic patterns in existing comments signal rationale.
*   **Technique:** Classifiers distinguish "operational" comments from "intent" comments.
    *   *Operational:* "Increments i by 1."
    *   *Intent:* "Loop ensures buffer is full."
*   **Research Insight:** Tools like "Docmatic" or "FailureDoc" use these heuristics to generate documentation specifically for failed test cases, explaining *why* the test exists and what a failure implies about the system state.

### 3. State-of-the-Art: LLMs and RAG
Current research is moving toward **Retrieval-Augmented Generation (RAG)**. Instead of asking an LLM to "summarize this code," the system:
1.  **Retrieves** relevant commit messages, issue tickets, and caller usage.
2.  **Prompts** the LLM: "Given this code and this background history, explain *why* this function is necessary."

**Summary of Research Landscape**
| Focus | "What" (Summarization) | "Why" (Rationale) |
| :--- | :--- | :--- |
| **Input Data** | Code tokens, AST | Commit logs, Issue trackers, Call graphs |
| **Key Models** | CodeBERT, CodeT5, seq2seq | RAG-based LLMs, Traceability Linkers |
| **Primary Challenge** | Accuracy, naming variable | Missing context, "hallucinating" intent |
| **Goal** | Readability | Maintainability & Onboarding |
