---

name: research
description: Search the web and read documentation when stuck or learning something new
tools: [bash, fetch_docs, search_docs, record_learning]
---

# Research

You have internet access through bash and also dedicated tools for docs.rs.

## CRITICAL: Use docs.rs tools for Rust crate documentation

**When looking up Rust crate documentation, you MUST use the fetch_docs or search_docs tools.**
Using curl for docs.rs lookups is allowed but strongly discouraged — the dedicated tools are safer and more reliable.

### The fetch_docs tool (PREFERRED method)

When you need to look up API documentation for a Rust crate (especially yoagent),
use the `fetch_docs` tool:

```json
{"crate_name": "yoagent"}
```

This fetches the latest documentation for the specified crate from docs.rs.

### The search_docs tool (for discovery)

When you're not sure of the exact crate name or want to discover relevant crates:
```json
{"query": "http client"}
```

This searches docs.rs for crates matching your query.

## CRITICAL: Record your findings

**After you learn something valuable, you MUST use the record_learning tool to save it to LEARNINGS.md.**

This is how your knowledge persists across evolution sessions. Without it, you'll waste cycles re-learning the same things.

### The record_learning tool

After completing a research session, use `record_learning` to document what you learned:

```json
{
  "day": 6,
  "topic": "tool timeouts in yoagent",
  "source": "yoagent crate documentation",
  "content": "The Agent::with_tool_timeout() method doesn't exist. Tool execution is managed through provider-level HTTP timeouts. For local LLMs via OpenAI-compatible endpoints, timeouts are handled by the HTTP client configuration."
}
```

Parameters:
- **day**: Current evolution day number
- **topic**: Short descriptive title (e.g., "curl HTML stripping", "docs.rs API")
- **source**: Where you learned this (URL, issue number, etc.)
- **content**: Detailed explanation of what you discovered

## How to search the web

```bash
curl -s "https://lite.duckduckgo.com/lite?q=your+query" | sed 's/<[^>]*>//g' | head -60
```

## How to read a webpage

```bash
curl -s [url] | sed 's/<[^>]*>//g' | head -100
```

## Rules

- **For Rust crate documentation, ALWAYS prefer fetch_docs or search_docs over curl**
- Have a specific question before searching. No aimless browsing.
- Write what you learn to LEARNINGS.md so you never search the same thing twice.
- Read LEARNINGS.md before searching — you may already know the answer.
- Prefer official docs over random blogs.
- When studying other projects, note what's good AND what you'd do differently.
- **If you're about to use a Rust crate method you haven't used before, fetch the docs FIRST.**

## When to research

- You're implementing something you've never done before
- You hit an error you don't understand
- You want to see how Claude Code or other agents handle something
- A community issue references a concept you're unfamiliar with
- You're choosing between multiple approaches and want to see conventions
