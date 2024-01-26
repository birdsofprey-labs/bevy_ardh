set OPENCV_IO_ENABLE_OPENEXR=1
python cuber.py hgt
python cuber.py nor
python spliterh.py gallery\hgt5.exr hgt
python spliter.py gallery\nor5.png nor