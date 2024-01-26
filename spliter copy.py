import cv2
import sys
from PIL import Image, ImageDraw, ImageFont
import numpy as np

Image.MAX_IMAGE_PIXELS = 933120000

def make_tile2(id, itype, hw):
    image = Image.new("RGB", (hw[0], hw[1]), "green")
    font = ImageFont.truetype("times.ttf", size= hw[0]//2)
    draw = ImageDraw.Draw(image)
    draw.text((10, 10), f"i{id}", font= font)
    image.save(f'tiles/{itype}{id}.png') 

def make_tile(id, itype,xy, hw):
    image = img
    cropped = image.crop((xy[0],xy[1],xy[0]+hw[0],xy[1]+hw[1]))
    #cropped = cropped.transpose(Image.FLIP_TOP_BOTTOM)
    if cropped.width > 512:
        cropped = cropped.resize((512,512))
    if itype == 'hgt':
        h = np.array(cropped)
        h = h.astype("float32")
        cv2.imwrite(f'assets/tiles/{itype}_{faceid}_{id}.exr', h)
    else:
        cropped.save(f'assets/tiles/{itype}_{faceid}_{id}.png') 

image_filename = sys.argv[1].strip()
save_as = sys.argv[2].strip()
faceid = sys.argv[3].strip()
img = Image.open(image_filename)

print('processing', image_filename)
width, height = img.size
#width, height = width // 2, height // 2
max_depth = 4
def split(depth, id, xy, wh):
    # = depth + 2 
    #xy = xy // 2
    D = wh // 2
    NW = xy
    NE = xy + np.array([wh[0] // 2, 0])
    SW = xy + np.array([0, wh[1] // 2])
    SE = xy + wh // 2
    id *= 4

    print(f'{id+1}', NW, D)
    print(f'{id+2}', NE, D)
    print(f'{id+3}', SW, D)
    print(f'{id+4}', SE, D)

    if depth < max_depth:
        split(depth+1, id+1, NW, D)
        split(depth+1, id+2, NE, D)
        split(depth+1, id+3, SW, D)
        split(depth+1, id+4, SE, D)
        
    # make_tile(id+3, save_as, NW, D)
    # make_tile(id+4, save_as, NE, D)
    # make_tile(id+1, save_as, SW, D)
    # make_tile(id+2, save_as, SE, D)
    make_tile(id+1, save_as, NW, D)
    make_tile(id+2, save_as, NE, D)
    make_tile(id+3, save_as, SW, D)
    make_tile(id+4, save_as, SE, D)
if __name__ == '__main__':
    split(1, 0, np.array([0, 0], dtype=int), np.array([width, height]) )


