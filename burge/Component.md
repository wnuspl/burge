Anything. Can be passed to elements via [[Component#Manager]]
```rust
trait Component {
	fn build_element(&self) -> Element;
	fn to_any(&self) -> Rc<std::any::Any>;
}
```


## Manager
Passed to elements and allows safe access of available components
Functionality:
 - Store built components
 - Building with [[JSON Manager]]
```json
"scenes": [
	{
		"components": [ "input", "physics" ]
	}
]

//JSON Manager will call the build_element method for "input" and "physics", adding it to the scene if != Element::Null
```
