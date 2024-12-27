
# PyO3 user guide

# Executing existing Python code

If you already have some existing Python code that you need to execute from Rust, the following FAQs can help you select the right PyO3 functionality for your situation:

## Want to access Python APIs? Then use PyModule::import.

PyModule::import can be used to get handle to a Python module from Rust. You can use this to import and use any Python
module available in your environment.

```
use pyo3::prelude::*;

fn main() -> PyResult<()> {
    Python::with\_gil(|py| {
        let builtins = PyModule::import(py, "builtins")?;
        let total: i32 = builtins
            .getattr("sum")?
            .call1((vec![1, 2, 3],))?
            .extract()?;
        assert\_eq!(total, 6);
        Ok(())
    })
}
```

## Want to run just an expression? Then use eval.

Python::eval is
a method to execute a Python expression
and return the evaluated value as a Bound<'py, PyAny> object.

```
use pyo3::prelude::*;
use pyo3::ffi::c\_str;

fn main() -> Result<(), ()> {
Python::with\_gil(|py| {
    let result = py
        .eval(c\_str!("[i * 10 for i in range(5)]"), None, None)
        .map\_err(|e| {
            e.print\_and\_set\_sys\_last\_vars(py);
        })?;
    let res: Vec<i64> = result.extract().unwrap();
    assert\_eq!(res, vec![0, 10, 20, 30, 40]);
    Ok(())
})
}
```

## Want to run statements? Then use run.

Python::run is a method to execute one or more
Python statements.
This method returns nothing (like any Python statement), but you can get
access to manipulated objects via the locals dict.

You can also use the py\_run! macro, which is a shorthand for Python::run.
Since py\_run! panics on exceptions, we recommend you use this macro only for
quickly testing your Python extensions.

```
use pyo3::prelude::*;
use pyo3::py\_run;

fn main() {
#[pyclass]
struct UserData {
    id: u32,
    name: String,
}

#[pymethods]
impl UserData {
    fn as\_tuple(&self) -> (u32, String) {
        (self.id, self.name.clone())
    }

    fn \_\_repr\_\_(&self) -> PyResult<String> {
        Ok(format!("User {}(id: {})", self.name, self.id))
    }
}

Python::with\_gil(|py| {
    let userdata = UserData {
        id: 34,
        name: "Yu".to\_string(),
    };
    let userdata = Py::new(py, userdata).unwrap();
    let userdata\_as\_tuple = (34, "Yu");
    py\_run!(py, userdata userdata\_as\_tuple, r#"
assert repr(userdata) == "User Yu(id: 34)"
assert userdata.as\_tuple() == userdata\_as\_tuple
    "#);
})
}
```

## You have a Python file or code snippet? Then use PyModule::from\_code.

PyModule::from\_code
can be used to generate a Python module which can then be used just as if it was imported with
PyModule::import.

Warning: This will compile and execute code. Never pass untrusted code
to this function!

```
use pyo3::{prelude::*, types::IntoPyDict};
use pyo3\_ffi::c\_str;

fn main() -> PyResult<()> {
Python::with\_gil(|py| {
    let activators = PyModule::from\_code(
        py,
        c\_str!(r#"
def relu(x):
    """see https://en.wikipedia.org/wiki/Rectifier\_(neural\_networks)"""
    return max(0.0, x)

def leaky\_relu(x, slope=0.01):
    return x if x >= 0 else x * slope
    "#),
        c\_str!("activators.py"),
        c\_str!("activators"),
    )?;

    let relu\_result: f64 = activators.getattr("relu")?.call1((-1.0,))?.extract()?;
    assert\_eq!(relu\_result, 0.0);

    let kwargs = [("slope", 0.2)].into\_py\_dict(py)?;
    let lrelu\_result: f64 = activators
        .getattr("leaky\_relu")?
        .call((-1.0,), Some(&kwargs))?
        .extract()?;
    assert\_eq!(lrelu\_result, -0.2);
   Ok(())
})
}
```

## Want to embed Python in Rust with additional modules?

Python maintains the sys.modules dict as a cache of all imported modules.
An import in Python will first attempt to lookup the module from this dict,
and if not present will use various strategies to attempt to locate and load
the module.

The append\_to\_inittab
macro can be used to add additional #[pymodule] modules to an embedded
Python interpreter. The macro must be invoked before initializing Python.

As an example, the below adds the module foo to the embedded interpreter:

```
use pyo3::prelude::*;
use pyo3::ffi::c\_str;

#[pyfunction]
fn add\_one(x: i64) -> i64 {
    x + 1
}

#[pymodule]
fn foo(foo\_module: &Bound<'\_, PyModule>) -> PyResult<()> {
    foo\_module.add\_function(wrap\_pyfunction!(add\_one, foo\_module)?)?;
    Ok(())
}

fn main() -> PyResult<()> {
    pyo3::append\_to\_inittab!(foo);
    Python::with\_gil(|py| Python::run(py, c\_str!("import foo; foo.add\_one(6)"), None, None))
}
```

If append\_to\_inittab cannot be used due to constraints in the program,
an alternative is to create a module using PyModule::new
and insert it manually into sys.modules:

```
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::ffi::c\_str;

#[pyfunction]
pub fn add\_one(x: i64) -> i64 {
    x + 1
}

fn main() -> PyResult<()> {
    Python::with\_gil(|py| {
        // Create new module
        let foo\_module = PyModule::new(py, "foo")?;
        foo\_module.add\_function(wrap\_pyfunction!(add\_one, &foo\_module)?)?;

        // Import and get sys.modules
        let sys = PyModule::import(py, "sys")?;
        let py\_modules: Bound<'\_, PyDict> = sys.getattr("modules")?.downcast\_into()?;

        // Insert foo into sys.modules
        py\_modules.set\_item("foo", foo\_module)?;

        // Now we can import + run our python code
        Python::run(py, c\_str!("import foo; foo.add\_one(6)"), None, None)
    })
}
```

## Include multiple Python files

You can include a file at compile time by using
std::include\_str macro.

Or you can load a file at runtime by using
std::fs::read\_to\_string function.

Many Python files can be included and loaded as modules. If one file depends on
another you must preserve correct order while declaring PyModule.

Example directory structure:

```
.
├── Cargo.lock
├── Cargo.toml
├── python\_app
│   ├── app.py
│   └── utils
│       └── foo.py
└── src
    └── main.rs
```

python\_app/app.py:

```
from utils.foo import bar

def run():
    return bar()
```

python\_app/utils/foo.py:

```
def bar():
    return "baz"
```

The example below shows:

- how to include content of app.py and utils/foo.py into your rust binary
- how to call function run() (declared in app.py) that needs function
imported from utils/foo.py

src/main.rs:

```
use pyo3::prelude::*;
use pyo3\_ffi::c\_str;

fn main() -> PyResult<()> {
    let py\_foo = c\_str!(include\_str!(concat!(
        env!("CARGO\_MANIFEST\_DIR"),
        "/python\_app/utils/foo.py"
    )));
    let py\_app = c\_str!(include\_str!(concat!(env!("CARGO\_MANIFEST\_DIR"), "/python\_app/app.py")));
    let from\_python = Python::with\_gil(|py| -> PyResult<Py<PyAny>> {
        PyModule::from\_code(py, py\_foo, c\_str!("utils.foo"), c\_str!("utils.foo"))?;
        let app: Py<PyAny> = PyModule::from\_code(py, py\_app, c\_str!(""), c\_str!(""))?
            .getattr("run")?
            .into();
        app.call0(py)
    });

    println!("py: {}", from\_python?);
    Ok(())
}
```

The example below shows:

- how to load content of app.py at runtime so that it sees its dependencies
automatically
- how to call function run() (declared in app.py) that needs function
imported from utils/foo.py

It is recommended to use absolute paths because then your binary can be run
from anywhere as long as your app.py is in the expected directory (in this example
that directory is /usr/share/python\_app).

src/main.rs:

```
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3\_ffi::c\_str;
use std::fs;
use std::path::Path;
use std::ffi::CString;

fn main() -> PyResult<()> {
    let path = Path::new("/usr/share/python\_app");
    let py\_app = CString::new(fs::read\_to\_string(path.join("app.py"))?)?;
    let from\_python = Python::with\_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath = py
            .import("sys")?
            .getattr("path")?
            .downcast\_into::<PyList>()?;
        syspath.insert(0, path)?;
        let app: Py<PyAny> = PyModule::from\_code(py, py\_app.as\_c\_str(), c\_str!(""), c\_str!(""))?
            .getattr("run")?
            .into();
        app.call0(py)
    });

    println!("py: {}", from\_python?);
    Ok(())
}
```

## Need to use a context manager from Rust?

Use context managers by directly invoking \_\_enter\_\_ and \_\_exit\_\_.

```
use pyo3::prelude::*;
use pyo3::ffi::c\_str;

fn main() {
    Python::with\_gil(|py| {
        let custom\_manager = PyModule::from\_code(
            py,
            c\_str!(r#"
class House(object):
    def \_\_init\_\_(self, address):
        self.address = address
    def \_\_enter\_\_(self):
        print(f"Welcome to {self.address}!")
    def \_\_exit\_\_(self, type, value, traceback):
        if type:
            print(f"Sorry you had {type} trouble at {self.address}")
        else:
            print(f"Thank you for visiting {self.address}, come again soon!")

        "#),
            c\_str!("house.py"),
            c\_str!("house"),
        )
        .unwrap();

        let house\_class = custom\_manager.getattr("House").unwrap();
        let house = house\_class.call1(("123 Main Street",)).unwrap();

        house.call\_method0("\_\_enter\_\_").unwrap();

        let result = py.eval(c\_str!("undefined\_variable + 1"), None, None);

        // If the eval threw an exception we'll pass it through to the context manager.
        // Otherwise, \_\_exit\_\_  is called with empty arguments (Python "None").
        match result {
            Ok(\_) => {
                let none = py.None();
                house
                    .call\_method1("\_\_exit\_\_", (&none, &none, &none))
                    .unwrap();
            }
            Err(e) => {
                house
                    .call\_method1(
                        "\_\_exit\_\_",
                        (
                            e.get\_type(py),
                            e.value(py),
                            e.traceback(py),
                        ),
                    )
                    .unwrap();
            }
        }
    })
}
```

## Handling system signals/interrupts (Ctrl-C)

The best way to handle system signals when running Rust code is to periodically call Python::check\_signals to handle any signals captured by Python's signal handler. See also the FAQ entry.

Alternatively, set Python's signal module to take the default action for a signal:

```
use pyo3::prelude::*;

fn main() -> PyResult<()> {
Python::with\_gil(|py| -> PyResult<()> {
    let signal = py.import("signal")?;
    // Set SIGINT to have the default action
    signal
        .getattr("signal")?
        .call1((signal.getattr("SIGINT")?, signal.getattr("SIG\_DFL")?))?;
    Ok(())
})
}
```