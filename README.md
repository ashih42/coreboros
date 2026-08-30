# Coreboros

Coreboros is a modern remake of the classic [Core War](https://en.wikipedia.org/wiki/Core_War) virtual machine, written in Rust.

You can play in the browser [HERE](https://ashih42.github.io/coreboros/).

## How to Run

Run the assembler (to verify Redcode files are valid).

```
cargo run --bin asm -- <warrior1.red> [ <warrior2.red> ... ]
```

Run the game as a native OS app.

```
cargo run --bin game
```

Run the game with an HTTP server to play in the browser.

```
./scripts/run-web-game.sh
```

## About Core War

- Q: What is Core War?
  - It is a game where you write a warrior program in the game's own Redcode assembly language. Then, you load multiple warriors into the game and watch them compete against each other, executing one instruction after another in a time-shared system with a shared circular memory space. With no further user input, you simply observe the simulation unfold as these warriors carry out their low-level strategies to stay alive and eliminate their competitors, until only one warrior remains as the winner.

- Q: How do I even begin with this game?
  - You can try the provided `dwarf` and `imp` warriors, run one step at a time, and observe how they affect the core.

- Q: Where can I find more information to learn to play this game?
  - Check out the [Resources](#resources) section below.

- Q: Where did Core War come from?
  - It was first described in an article in the May 1984 issue of [Scientific American](https://www.scientificamerican.com/) magazine. You can find snipplets of these articles [HERE](https://www.corewars.org/sciam/).

- Q: Which version of Core War is this project based on?
  - This project follows the [ICWS'94 Standard](https://corewar.co.uk/standards/icws94.htm). If you come from 42's variant of Core War, that knowledge won't help you here.

## Current Limitations

- Currently, Redcode macros (e.g. `EQU`, `FOR` loops) are not supported. However, you can still use [pMARS](http://www.koth.org/pmars/) to compile your Redcode to a load file format without macros, and then load that into Coreboros.

- Currently, p-space operations (e.g. `LDP`, `STP`) are not implemented. Although Coreboros accepts these as valid opcodes, they currently have no effect, just like `NOP`.

## Possibly Upcoming Features

- Add a game log at bottom of screen (in Arena scene) to show who killed whom.
  - Apply color highlighting around each Warrior's name.

- Add a toggle to favorite/unfavorite warriors in menu.
  - Show favorited warriors, followed by non-favorited warriors in alphabetical order.

- Add a dropdown to assign specific color to warrior (instead of current fixed color assignment by order in queue).

## Hey You Implemented X Wrong!

Feel free to tell me where I am wrong. A Github issue with an example detailing the input and expected output would be super helpful. You are an even bigger giga chad if you can explain with pMARS side-by-side on what the correct result should be.

## References

- [Beginner's Guide to Redcode](https://corewars.org/docs/guide.html)
  - This is an excellent introduction. Read this first.

- [Corewar.io Documentation](https://corewar-docs.readthedocs.io/en/latest/)
  - This is a nice, clean documentation, though missing some details.

- [ICWS'94 Standard](https://corewar.co.uk/standards/icws94.htm)
  - This document is not only incomplete, but also not even officially adopted! (ノಠ益ಠ)ノ彡┻━┻

- [pMARS Manual](https://corewar.co.uk/pmars/pmars_man.htm)
  - This is a quick reference on how pMARS does things.

- [Redcode Strategies](https://corewar.co.uk/strategy.htm)
  - This is a compendium of many different warrior strategies, with code and explanation.

- [n1LS's Collection of Redcode Warriors](https://github.com/n1LS/redcode-warriors)
  - Hmmm... 🤔
