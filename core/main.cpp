#include <dlapi.h>

#include "funcs.h"
#include "utils.h"

int main() {
	auto pGateway = dl::getGateway();

	/* 	// create the file and primary image */
	/* 	fits_create_file(&fptr, "sample.fits\0", &status); */
	/* 	fits_create_img(fptr, USHORT_IMG, naxis, naxes, &status); */

	/* 	// Pull binning and offset information from the suframe and add them to the header */
	/* 	// Set the headers */
	/* 	fits_update_key(fptr, TFLOAT, "EXPOSURE", &options.duration, */
	/* 		"Total Exposure Time", &status); */
	/* 	fits_update_key(fptr, TUINT, "BINNING", &subf.binX, */
	/* 		"Image Bin Level", &status); */
	/* 	fits_update_key(fptr, TUINT, "XOFFSET", &subf.left, */
	/* 		"X Axis offset", &status); */
	/* 	fits_update_key(fptr, TUINT, "YOFFSET", &subf.top, */
	/* 		"Y Axis Offset", &status); */
	/* 	fits_write_date(fptr, &status); // exposure timestamp header */

	/* 	// write the image and close the file */
	/* 	fits_write_img(fptr, TUSHORT, fpixel, nelements, pBuffer, &status); */
	/* 	fits_close_file(fptr, &status); */
	/* } */

  auto camera = initialize_camera(pGateway).expect("Could not initialize camera!");
  ExposureInfo expinfo;
  expinfo.bin_x = 1;
  expinfo.bin_y = 1;
  expinfo.duration = 0.2;
  expinfo.light = true;
  expinfo.readout_mode = ReadoutMode::High;
  auto er = expose(camera, expinfo).expect("Could not expose!");

  std::cout << er.buffer << " " << er.buffer_size << std::endl;

	free_gateway(pGateway);
	return 0;
}
