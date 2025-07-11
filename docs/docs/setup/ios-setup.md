# iOS Setup

This tutorial will guide you through integrating the iOS bindings into an Xcode project. Before you begin, make sure you’ve completed the ["Getting Started - 3. Mopro build"](/docs/getting-started.md#3-build-bindings) process with selecting **iOS** platform and have the `MoproiOSBindings` folder ready:

```sh
MoproiOSBindings
├── mopro.swift
└── MoproBindings.xcframework
```

Watch the demo video below for a step-by-step guide to integrating the bindings into Xcode, or follow the written instructions that follow.

<p align="center">
<iframe width="560" height="315" src="https://www.youtube.com/embed/6TydXwYMQCU?si=TDw5qkbWSs-Uhw5E" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</p>

:::info
In this example, we use Circom circuits and their corresponding `.zkey` files. The process is similar for other provers.
:::

## 0. Initialize an XCode project

First let's make a new iOS app Xcode project. If you already have an app project you can skip this step. We'll do `File` -> `New` -> `Project` and create an iOS App. Make sure the language is _Swift_. We suggest putting this iOS project inside the rust project folder created above.

<p align="center">
![create an ios app project](/img/ios-example-1.png)
</p>

## 1. Drag the `MoproiOSBindings` folder into the project

Your Xcode project should be open now. Open **Finder** and drag the `MoproiOSBindings` folder into the project folder structure.

<p align="center">
![add mopro ios bindings to project](/img/ios-example-2.png)
</p>

## 2. Place proving keys into the project

Next drag in any key you plan to prove with. Go to the project **Build Phases** and add each key to the **Copy Bundle Resources** step.

![copy zkeys as bundle resources](/img/ios-example-3.png)

Now you're ready to write the proving code in your app!

:::warning
Although relative paths may work locally in Rust, the proving keys should be copied into the project to ensure they are accessible by the mobile app.
:::

## 3. Proving from the app

In your project there should be a file named `ContentView`. Add a private variable and a button like this:

```swift
struct ContentView: View {
    private let zkeyPath = Bundle.main.path(forResource: "multiplier3_final", ofType: "zkey")!

    var body: some View {
        VStack {
            Button("Prove", action: runProveAction)
        }
        .padding()
    }
}
```

We use the `Bundle` api to retrieve the full path to our zkey. Change `multiplier3_final` to the name of your zkey.

At the bottom of this file we'll add an extension with a function to generate a proof. In this example we're going to prove a simple circuit that accepts two inputs named `a` and `b` and generates an output `c`.

```swift
extension ContentView {
    func runProveAction() {
        // Prepare inputs
        //
        // The generateCircomProof function accepts an absolute path
        // to the zkey, and a map of strings to arrays of strings
        //
        // This is a mapping of input names to values. Note that if
        // the input is not an array, it will still be specified as
        // and array of length 1.
        let input_str: String = "{\"b\":[\"5\"],\"a\":[\"3\"]}"

        // Begin timing our proof generation
        let start = CFAbsoluteTimeGetCurrent()

        // Call into the compiled static library
        do {
            let generateProofResult = try generateCircomProof(zkeyPath: zkeyPath, circuitInputs: input_str, proofLib: ProofLib.arkworks)
        } catch {
            print("Error generate a proof: \(error)")
        }

        let end = CFAbsoluteTimeGetCurrent()
        let timeTaken = end - start
        print("built proof in \(String(format: "%.3f", timeTaken))s")
    }
}
```

You should now be able to run the iOS app on the simulator or a device and build a proof. The app should log the time taken to generate the proof. For a more complete example including serialization and verification check [here](https://github.com/zkmopro/mopro/blob/63d6cbaf1bdbcb089f889f86b2a6a0443c6a8679/test-e2e/ios/mopro-test/ContentView.swift).

## 4. What's next?

-   **Update your ZK circuits** as needed. After making changes, be sure to run:

    ```sh
    mopro build
    mopro update
    ```

    :::warning
    `mopro update` only works if the Android project was created _within_ the Rust project directory during mopro init. Otherwise, you can manually update the bindings by following [Step 1](#1-drag-the-moproiosbindings-folder-into-the-project).
    :::

    This ensures the bindings are regenerated and reflect your latest updates.

-   **Build your mobile app frontend** according to your business logic and user flow.
-   **Expose additional Rust functionality:**
    If a function is missing in Swift, Kotlin, React Native, or Flutter, you can:

    -   Add the required Rust crate in `Cargo.toml`
    -   Annotate your function with `#[uniffi::export]` (See the [Rust setup](rust-setup.md#-customize-the-bindings) guide for details).<br/>
        Once exported, the function will be available across all supported platforms.

## 5. Importing multiple bindings with static libraries

Adding two or more Mopro-generated bindings directly into a single Xcode project will result in duplicate symbol errors.  
Fix this by isolating each binding in its own static-library target, then linking those targets to the app.

1. **Create a static-library target**  
   `File → New → Target → iOS → Framework & Library → Static Library` — name it `MyLibraryA`.

2. **Delete the autogenerated Swift source file** in `MyLibraryA`.

3. **Drag the binding folder** (`MoproiOSBindings`) into Xcode and tick **only** the `MyLibraryA` target in the *Add Files* dialog.

4. **Expose the library to the app**  
   *App target → General → Frameworks, Libraries & Embedded Content* → **+** → `libMyLibraryA.a`.  
   Remove any `MoproBindings.xcframework` already listed to avoid duplicate symbol errors.

5. **Repeat steps 1–4** for each additional binding (`MyLibraryB`, `MyLibraryC`, …).  
   Ensure each binding was generated from a distinct Mopro project with a unique name.

You can now `import` and call functions from every binding without linker conflicts.