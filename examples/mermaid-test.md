# Mermaid Diagram Test

This document demonstrates various types of Mermaid diagrams.

## Flowchart

```mermaid
graph TD
    A[Start] --> B{Is it working?}
    B -->|Yes| C[Great!]
    B -->|No| D[Debug]
    D --> E[Fix Issues]
    E --> B
    C --> F[End]
```

## Sequence Diagram

```mermaid
sequenceDiagram
    participant User
    participant Browser
    participant Server
    User->>Browser: Request page
    Browser->>Server: HTTP GET
    Server->>Browser: HTML + Mermaid.js
    Browser->>User: Render diagram
    User->>Browser: View diagram
```

## Gantt Chart

```mermaid
gantt
    title Mermaid Implementation Schedule
    dateFormat  YYYY-MM-DD
    section Planning
    Create plan           :done, plan, 2025-11-06, 1d
    section Implementation
    Create module         :done, module, 2025-11-06, 2h
    Wire up code          :done, wire, 2025-11-06, 1h
    section Testing
    Create tests          :active, test, 2025-11-06, 1h
    Browser testing       :verify, after test, 1h
```

## Class Diagram

```mermaid
classDiagram
    class Mermaid{
        +initialize()
        +render()
        +String theme
        +String securityLevel
    }
    class Markdown{
        +parse()
        +renderToHTML()
    }
    class CodeBlock{
        +String language
        +String content
    }
    Markdown --> CodeBlock
    CodeBlock --> Mermaid : language==mermaid
```

## State Diagram

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Loading : User opens page
    Loading --> Rendering : Mermaid.js loaded
    Rendering --> Displayed : Diagrams rendered
    Displayed --> [*]
    Loading --> Error : Load failed
    Rendering --> Error : Syntax error
    Error --> [*]
```

## Pie Chart

```mermaid
pie title Code Distribution
    "Rust" : 45
    "JavaScript (Mermaid)" : 35
    "HTML/CSS" : 15
    "Markdown" : 5
```

## Entity Relationship Diagram

```mermaid
erDiagram
    MARKDOWN ||--o{ CODE_BLOCK : contains
    CODE_BLOCK ||--o| MERMAID_DIAGRAM : renders
    CODE_BLOCK {
        string language
        string content
        int line_number
    }
    MERMAID_DIAGRAM {
        string type
        string svg_output
    }
```

## User Journey

```mermaid
journey
    title User experience with Mermaid diagrams
    section Writing
      Write markdown: 5: User
      Add mermaid block: 4: User
    section Building
      Run wool: 5: Tool
      Process markdown: 5: Tool
      Inject Mermaid.js: 5: Tool
    section Viewing
      Open in browser: 5: User
      View rendered diagrams: 5: User
```

## Mixed Content

Regular markdown content can coexist with Mermaid diagrams.

Here's some **bold text** and *italic text*.

- List item 1
- List item 2
- List item 3

And here's another diagram:

```mermaid
graph LR
    A[Markdown] --> B[HTML]
    B --> C[Browser]
    C --> D[Rendered Content]
```

Regular code blocks still work fine:

```rust
fn main() {
    println!("Hello, world!");
}
```

```python
def greet(name):
    return f"Hello, {name}!"
```

## Edge Cases

### Empty Diagram Block

```mermaid
```

### Invalid Syntax (should show error)

```mermaid
this is not valid mermaid syntax
```

### Multiple Diagrams Side by Side

```mermaid
graph LR
    A --> B
```

```mermaid
graph TB
    C --> D
```

## Conclusion

This test file demonstrates that Mermaid diagrams integrate seamlessly with standard Markdown content.
