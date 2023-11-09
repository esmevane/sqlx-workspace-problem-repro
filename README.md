# SQLX Workspace Issues Reproduction Repo

This repo is a reproduction of a handful of issues I'm having with SQLX in a workspace.

The overall idea (and I think the root cause of my problem) is that I have a workspace which implements queries against two database adapters: postgres and sqlite.

In order to work with these adapters, I build two versions of the same queries and then leverage them and re-export them from a third crate. This is sort of a diamond pattern, where the two database adapters are at the bottom of the diamond, and the third crate is at the top.

Having multiple database adapters in a single workspace seems to break a lot of SQLX's assumptions.

1. The project root seems to want a DATABASE_URL, but I don't want to set one because I have two databases, each with crate-specific DATABASE_URLs.
2. SQLite wants to know about a DATABASE_URL which is relative to the current working directory, which again isn't possible because I have two databases.
3. The offline modes don't seem to respond to any approach I take to test, seemingly ignoring env variables entirely unless I provide `DATABASE_URL`, which it then takes seriously even if I attempt to force it to be offline with `SQLX_OFFLINE=true`.

It seems like the only viable ways around this would be to split apart my workspace, either just in tests (setting up some elaborate test workarounds) or in the code itself (which would be a lot of work and would make it harder to share code between the two adapters). I'd like to be able to deploy a few versions of my code against different ephemeral/concrete databases, and it'd be nice to keep all of that code in one place.

## Things which break

```bash
cargo sqlx prepare --workspace
```

This breaks in the project root because it wants a single database adapter, not two.
