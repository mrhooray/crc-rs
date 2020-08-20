#!/bin/sh

echo "use crate::util::*;"

emit_table_gen() {
  width=$1
  echo "pub(crate) const fn crc${width}_table(poly: u${width}, reflect: bool) -> [u${width}; 256] {"
  echo "let mut table = [0u${width}; 256];"
  i=0; while [ $i -le 255 ]; do  
    echo "table[${i}] = crc${width}(poly, reflect, ${i});"
    i=$(( i + 1 ))  
  done
  echo "table"
  echo "}"
}

emit_table_gen 16
emit_table_gen 32
emit_table_gen 64
