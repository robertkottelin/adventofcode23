# adventofcode23

Ho-Ho-Ho! Implementing solutions in dotnet and rust.

## .NET

Common, reusable functions are stored inside cs/AdventOfCode.common/

Add individual day projects:
```
dotnet new console -n AdventOfCode.Day01
dotnet sln AdventOfCode.sln add AdventOfCode.Day01/AdventOfCode.Day01.csproj
dotnet add AdventOfCode.Day01/AdventOfCode.Day01.csproj reference AdventOfCode.Common/AdventOfCode.Common.csproj
```

Build & Run:
```
dotnet build AdventOfCode.sln
dotnet run --project AdventOfCode.Day01
```

## Rust

Common, reusable functions are stored inside rust/common/


Add individual day projects:
```
cargo new day_1
```

Build & Run:
```
cargo build
cargo run
```

Data is stored in a folder called "data" inside the root project for both .NET and Rust implementations