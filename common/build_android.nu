let renamedTargets = {
  aarch64-linux-android:   arm64-v8a
  armv7-linux-androideabi: armeabi-v7a
  x86_64-linux-android:    x86_64
}

let targets = [
  aarch64-linux-android
  x86_64-linux-android
  armv7-linux-androideabi
]



def _build [ pkg target mode ] {

  # cargo install cargo-ndk
  cargo ndk build -p $pkg --target $target --($mode)
  
}

def build [ pkg targets mode? = 'release' ] {

  $targets
  | each {|target|
    _build $pkg $target $mode
  }

}

def generate_ffi_bindings [ pkg targets out mode? = 'release' ] {

  $targets
  | each {|target|
    let lib = $"target/($target)/($mode)/lib($pkg).so"
    (
      cargo run -p uniffi-bindgen -- 
      generate
      --library $lib
      --language kotlin
      --out-dir $out
    )
  }
 
}

def move_ffi_bindings [ pkg out module_dir ] {

  let dest = $"($module_dir)/kotlin"
  rm -rf $"($dest)/uniffi"
  mv $"($out)/uniffi" $dest

}

def move_to_android_module [ pkg targets module_dir mode? = 'release'] {

  $targets
  | each {|target|

    let dest = $"($module_dir)/jniLibs/($renamedTargets | get $target)"
    let lib = $"lib($pkg).so"
    let lib_file = $"target/($target)/($mode)/($lib)"

    if ($dest | path exists) {
      rm -r $dest
    }

    mkdir -v $dest
    mv $lib_file $"($dest)/($lib)"

  }

}

let package = 'calendarcore'
let mode = 'release'
let out = 'target/android'
let android_module_dir = '../android/calendar-core/src/main'

rm -rf $out
mkdir $out

build $package $targets
generate_ffi_bindings $package $targets $out $mode
move_ffi_bindings $package $out $android_module_dir
move_to_android_module $package $targets $android_module_dir $mode
