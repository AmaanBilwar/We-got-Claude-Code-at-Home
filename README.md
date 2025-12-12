## inspiration
1. [primeagen's video](https://www.youtube.com/watch?v=MNvciiZ5x9A)
2. [primeagen's copilot](https://www.youtube.com/watch?v=GuRiTCX-rT0)

## interesting topics to research
1. behavior trees

## how to? 
- use tree sitter to find smallest function using cursor location

## future plans
- make it into an lsp


## implementation

1. trigger manually with a shortcut when cursor is inside a function
2. use tree-sitter to find exact function boundaries
3. Sends only that function's signature to an LLM via async-llm integration
4. Streams the completion back asynchronously (non-blocking)
5. Returns a single TextEdit that replaces the incomplete function body
6. token limit for no extra code
