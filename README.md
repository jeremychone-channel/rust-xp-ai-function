Full tutorial on coding AI function calling in Rust programming by [Jeremy Chone](https://youtube.com/jeremychone). We will be using the OpenAI API, but similar approaches can be applied with Mistral, Mixtral, LLAMA, Gemini, and many other LLMs.

See [YoutTube Video AI Function Calling In Rust - Full Tutorial (OpenAI)](https://www.youtube.com/watch?v=2M0PSijLnis&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)

More on https://rust10x.com

## Other utils

Some bash/zsh aliases/functions

```sh
# Cargo watch quiet clear (user need to add the -x)
alias cw="cargo watch -q -c"

# Cargo watch run
alias cwr="cargo watch -q -c -w src/ -x 'run -q'"

# cargo watch example zsh/back function
# usage `cwe xp_file_name`
function cwe() {
  cargo watch -q -c -x "run -q --example '$1'"
}

# cargo watch test zsh/bash function
# usage `cwt test_my_fn`
function cwt() {
  cargo watch -q -c -x "test '$1' -- --nocapture"
}

# Cargo watch install
function cwi() {
  cargo watch -x "install --path ."
}
```

## Related links

- [GitHub Repo](https://github.com/jeremychone-channel/rust-xp-ai-function)
- [YouTube Video](https://www.youtube.com/watch?v=2M0PSijLnis&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q)