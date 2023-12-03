# adventofcode23

https://adventofcode.com/

Ho-Ho-Ho! Implementing solutions in dotnet and rust.

[![dotNET](https://cdn3.emoji.gg/emojis/6774_dotNET.png)](https://emoji.gg/emoji/6774_dotNET)

Common, reusable functions are stored inside cs/AdventOfCode.common/

Import them inside the individual day project:
```
using AdventOfCode.Common;
```

### Add individual day projects:
```
dotnet new console -n AdventOfCode.Day01
dotnet sln AdventOfCode.sln add AdventOfCode.Day01/AdventOfCode.Day01.csproj
dotnet add AdventOfCode.Day01/AdventOfCode.Day01.csproj reference AdventOfCode.Common/AdventOfCode.Common.csproj
```

### Build & Run:
```
dotnet build AdventOfCode.sln
dotnet run --project AdventOfCode.Day01
```

[![rust](https://cdn3.emoji.gg/emojis/4504-rust.png)](https://emoji.gg/emoji/4504-rust)

Common, reusable functions are stored inside common/src/

Import them inside the individual day Cargo.toml:
```
[dependencies]
common = { path = "../common" }
```

### Add individual day projects:
```
cargo new day_1
```

### Build & Run:
```
cargo build
cargo run
```

Data is stored in a folder called "data" inside the root project for both .NET and Rust implementations
