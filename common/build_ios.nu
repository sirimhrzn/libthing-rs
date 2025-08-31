const targets = [
  "aarch64-apple-ios-sim"
  "x86_64-apple-ios"
  # "aarch64-apple-ios"
]
const package = "calendarcore"
const bindings_out = "target/uniffi-xcframework-staging"

def build [
  pkg: string
  target: string
  mode?: string = 'release' ] {
  print $"Starting ($mode) bulid process for ($pkg) ($target)"
  cargo build -p $pkg --release --target $target
  print (ansi green) $"Build completed for ($pkg) ($target)" (ansi reset)
}



def build_package [pkg: string] {

  $targets
  | each {|target|
    build $pkg $target 'release'
  }

}


def generate_swift_ffi [ pkg, bindings_out ] {

  print $"Generating Swift FFI for ($pkg)"
  let module_name = $"($pkg)FFI"
  $targets
  | each { |target|
  (
      cargo run -p uniffi-bindgen-swift -- $"target/($target)/release/lib($pkg).a" $bindings_out
        --swift-sources
        --headers
        --modulemap
        --module-name $module_name
        --modulemap-filename module.modulemap
    )
  }

  print (ansi green) "Swift FFI generated ." (ansi reset)

}

def generate_fat_sim_lib [ pkg ] {

  let dir = "target/ios-simulator-fat/release"
  mkdir -v $dir

  print "Creating a Fat binary."

  (
    lipo -create
    $"target/x86_64-apple-ios/release/lib($pkg).a"
    $"target/aarch64-apple-ios-sim/release/lib($pkg).a"
    -output $"($dir)/lib($pkg).a"
  )

  print (ansi green) "Fat binary created." (ansi reset)
}

def generate_xcframework [ pkg bindings_out ] {

  print "Generating XCFramwork..."

  let out = "target/ios"
  rm -rf target/ios
  (
    xcodebuild -create-xcframework
    -library $"target/ios-simulator-fat/release/lib($pkg).a" -headers $bindings_out
    # -library $"target/aarch64-apple-ios/release/lib($pkg).a" -headers $bindings_out
    -output $"($out)/lib($pkg)-rs.xcframework"
  )

  print (ansi green) $"Generated XCFramwork at ($out)" (ansi reset)

}

def move_ffi_to_package [ pkg bindings_out ] {
  let dir = $"../apple/($pkg)"

  print "Moving Swift FFI to package."

  mkdir -v $dir
  glob $"($bindings_out)/*.swift" | each {|f| mv $f $dir}

  print (ansi green) "Swift FFI moved to package." (ansi reset)

}

build_package $package
generate_swift_ffi $package $bindings_out
generate_fat_sim_lib $package
generate_xcframework $package $bindings_out
move_ffi_to_package $package $bindings_out
