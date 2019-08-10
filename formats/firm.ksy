meta:
  id: firm_3ds
  file-extension: firm
  endian: le
  ks-opaque-types: true

seq:
  - id: header
    type: header

types:
  header:
    seq:
      - id: magic
        contents: 'FIRM'
      - id: priority
        type: u4
      - id: arm11_entry
        type: u4
      - id: arm9_entry
        type: u4
      - id: reserved
        size: 0x30
      - id: section_headers
        type: section_header
        repeat: expr
        repeat-expr: 4
      - id: signature
        size: 0x100

  section_header:
    seq:
      - id: offset
        type: u4
      - id: address
        type: u4
      - id: size
        type: u4
      - id: copy_method
        type: u4
        enum: copy_method
      - id: sha256
        size: 32
    instances:
      section:
        io: _root._io
        pos: offset
        size: size

enums:
  copy_method:
    0: ndma
    1: xdma
    2: cpu
