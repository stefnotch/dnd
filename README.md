# dnd - Do not disturb

A rusty Windows program to turn on focus mode for a set amount of time. It also logs every invocation of this program.

It's mainly a tool for me so that I can type things like

```
dnd 30m hard at work
```

and then not get disturbed for 30 minutes. And later down the road, I can check the logs to see how long I probably worked.

This program relies on some stuff being enabled in the focus mode settings (Game or fullscreen, not sure). It's also one terrible hack, since it calls some undocumented Windows APIs.
