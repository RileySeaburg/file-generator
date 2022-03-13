use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.tsx")?;
    file.write_all(b"
    
type Props = {}

function ReactComponent({}: Props) {
  return (
    <div>ReactComponent</div>
  )
}

export default ReactComponent")?;
    Ok(())
}