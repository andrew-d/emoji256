# emoji256

[![Build Status](https://travis-ci.org/andrew-d/emoji256.svg?branch=master)](https://travis-ci.org/andrew-d/emoji256) [![Build status](https://ci.appveyor.com/api/projects/status/9hnetjkflm8naxr4?svg=true)](https://ci.appveyor.com/project/andrew-d/emoji256)

This is a super-simple program that implements a base256 encoding using emoji.
It will encode anything passed to it on standard input, and write the output to
standard output.  The current emoji table is as follows:

👍👎👊✌✋👌👏👋👆👇👈👉✍👁👂👃  
👣🤖❌➕➖➗✔❗❓⁉🎌💯💋💍💎❤  
🎸🎷🎹🎻🏈🏀⚽⚾🏆🏁🏹🏅🏐☁🌪🌧  
🌩➰🐔🐧🐋🦀🐒🐑🐍🐎🐘🐙🐢🐝🐖🐊  
🐁🐄🐦🐌🐫🐬🐉🕷🕸🐈🐇🐜🐟🍣🍨🍩  
🍪🍫🍭🍔🍕🍞🎂🌮🌭🧀🍙🍝🍿🍎🍍🍌  
🍇🍉🍒🍓🍊🍋🍑🍐🍄🍅🍆🌶🌽🍷🍺☕  
🍸🍾🎃🎄🎅🎁☃❄⌚⌛⏰☎🎈🎉🎊🎆  
👻💀👿👽☀🌈🌙☔⭐🌵🌹🌻🍀🍁🌱🌴  
💄💅🎩🎤🎥🎨🎲⚠📷💰💳💲♠♣♥♦  
🚀🚒🚗🚢🚫🚲🚜🚁✈🚦💾💿📡📖📅📋  
📎📏📌✏✂🔍🔑🔒🔪🔫🔧🔨🍴🔥💣🚬  
👠👟👕👖👙👗👔👑👓👜💩🚽🚿🛀🎓🌋  
⛪🆗🎀💊💉🔔🔬🕯◀▶⬅⬆⬇↗↖↘  
↙↩↪🔄⏩⏪⏫⏬⏸✨☮☢☯✡⚓⚙  
🎢🎡🎪🚩🎬🎮🎰🎱🎵🎺🎿🏋🏭👅👀👯  

### Example

```
$ echo 'my string' | cargo run
🍷⌛🎸🎄🎅🎃🍅🍺🍐👈
```

Some possible real-world uses of this could include:

#### SSH Key Validation

```
$ ssh-keygen -E sha256 -l -f ~/.ssh/id_main_rsa | awk -F'[ :]' '{print $3}' | base64 -d | emoji256
⭐👿🍪📋✈✔✔🎁🎊🍞↪📷☔💣🔑🏋👀🍪🍸🍪✂🎻↘🌮🌽🎥💳🚽↪⏰🐫⌛
```

#### Hash Comparison

```
$ sha256sum Cargo.toml | cut -d' ' -f1 | xxd -r -p | emoji256
🤖🐊🐢↩🕸🐟♣☀↙🍄⬅🍸👯⏰👑✂🔍🍅🎰❓🍉📏🌵🏈❤🎂🌮🎈⛪👇🔧🎂
```

#### GPG Key Fingerprints

```
$ gpg --list-keys --with-colons pacman@localhost | grep '^fpr' | cut -d':' -f10 | xxd -r -p | emoji256
🍭🐦☕🏐✍🎁👣✔◀🎂💅🎊🌴🍝➖🌽🍣🎓♦🍁
```
