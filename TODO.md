- [ ] Take in and parse a url
- [ ] deduce the encodeing that is needed for it
- [ ] add the data to a bitmap 
- [ ] find out the needed poins for the formatting bits as well as the masking bits etc
  - [ ] grouding points
    - [ ] variability in grounding points with qr code size
      - should it be dynamic or need to be specified? perhaps dynamic by default and then also have flags that can be uses to specify if need be
  - [ ] masking bits
  - [ ] formatting bits
- [ ] return the qr code in a bitmap or other image format
  - bitmap may be the easiest as it is litterally all bits meaning if it's encoded with one bit per color meaning a 1 or a 0 white or black then it may be possible to pass that in as a qr code
  
## A note
--- 
There is already a library for rust that allows the creation of QR codes so perhaps we could make something that increases productivity with qr codes as the means 
as they are "quick response codes"