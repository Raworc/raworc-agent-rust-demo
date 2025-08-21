# Pirate-Speaking Raworc Agent Demo

Simple Pirate-speaking agent powered by Claude that responds to all messages in colorful Pirate language.

## Architecture

- **Zero Dependencies**: No raworc imports needed
- **Pure Function**: Just implement `process_message(message, context) -> response`
- **Claude Integration**: Direct Anthropic API integration using reqwest HTTP client
- **Language Focus**: Always responds like a stereotypical pirate
- **Git Deployment**: Deployed directly from GitHub repository

## Files

- `raworc.json` - Agent manifest (required)
- `src/lib.rs` - Agent implementation with `process_message` function
- `src/main.rs` - Binary entry point for testing
- `Cargo.toml` - Rust dependencies

## Agent Capabilities

This Pirate agent can:
- Respond to any question or request in colorful pirate language
- Handle any topic but always speak like a pirate
- Use authentic pirate vocabulary and expressions
- Provide helpful responses with pirate flair
- Demonstrate Rust async/await patterns with HTTP clients

## Example Interactions

**Input (any language):**
- "Hello!"
- "How are you today?"
- "Tell me about sailing"
- "What is treasure?"
- "Explain the ocean"

**Output (always Pirate):**
- Responds with "Arrr!", "Ahoy matey!", "Shiver me timbers!"
- Uses pirate vocabulary: ye, matey, scurvy dog, landlubber
- Enthusiastic and colorful pirate speech
- Maintains helpful content with pirate personality

## Testing Locally

```bash
export ANTHROPIC_API_KEY="your-api-key"
cargo build --release
cargo run
```

This will run test cases with pirate-themed interactions.

## Deploying to Raworc

1. Create agent in your space:
```bash
raworc api spaces/demo/agents --method POST --body '{
  "name": "pirate-agent",
  "description": "Responds in Pirate language using Claude LLM",
  "purpose": "demonstrate Pirate-speaking agent with Rust runtime",
  "source_repo": "Raworc/raworc-agent-rust-demo",
  "source_branch": "main"
}'
```

2. Build the space:
```bash
raworc api spaces/demo/build --method POST
```

3. Create session and test:
```bash
raworc api sessions --method POST --body '{"space": "demo"}'
```

4. Send messages:
```bash
raworc api sessions/{session_id}/messages --method POST --body '{
  "content": "Ahoy there! Tell me about treasure hunting.",
  "role": "user"
}'
```

The agent will automatically respond with enthusiastic pirate language!

## Key Features

- ✅ **High Performance**: Rust runtime with pre-compiled binaries
- ✅ **Async HTTP**: Modern reqwest client with tokio runtime
- ✅ **Language Consistency**: Always pirate speech output
- ✅ **Claude Powered**: Latest Claude 3.5 Sonnet model
- ✅ **Auto-Building**: cargo build runs automatically during space build
- ✅ **Fast Execution**: Pre-compiled Rust binary for instant responses

## Pirate Language Examples

The agent uses authentic pirate expressions:
- `Arrr!` - General pirate exclamation
- `Ahoy matey!` - Hello friend
- `Shiver me timbers!` - Expression of surprise
- `Avast!` - Stop/Pay attention
- `Ye scurvy dog!` - Friendly insult
- `Blimey!` - Expression of surprise
- `Batten down the hatches!` - Prepare for trouble

## Rust Implementation Notes

- **Async/Await**: Full async support with tokio runtime
- **HTTP Client**: reqwest for calling Claude API
- **Error Handling**: Proper Result types and error propagation
- **JSON Serialization**: serde_json for API communication
- **Sync Wrapper**: Compatible with both async and sync execution contexts