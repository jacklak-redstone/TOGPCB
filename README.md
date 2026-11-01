# That One General Purpose Chatto Bot (TOGPCB)

TOGPCB is aimed at one day being the [YAGPDB](https://yagpdb.xyz) for [Chatto](https://chatto.run). Currently, it only supports custom commands.

## Setup

Rename `.env.example` to `.env`, and set `TOKEN` and `SERVER` to your bot's token and the Chatto server (e.g. `https://chat.chatto.run`). Then do `cargo run --release` (you need a Rust toolchain).

## Custom Commands

They live in `templates/`, and types of custom command go in their respective folders (`command` for commands, `reaction_added` for reactions added, and `reaction_removed` for reactions removed).
Currently, only `command` type is supported. Also, all subfolders **MUST** be present, although they may be empty.

### Example Command

```jinja2
{% set user = get_user(args) %}
{% if user %}
    **User Info**
    Name: `{{ user.login }}`
    Display Name: `{{ user.displayName }}`
    Avatar URL: `{{ user.avatarUrl | default("unknown") }}`
    ID: `{{ user.id }}`
{% else %}
    No user with the ID `{{ args }}` could be found!
{% endif %}
```

Note: fields use **camelCase** instead of **snake_case**. Also, extra whitespace is trimmed. It uses MiniJinja, which is mostly faithful to Jinja2.
If you want more examples, check out the `templates/` folder. It contains semi-useful commands.
