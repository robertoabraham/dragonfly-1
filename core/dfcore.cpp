#include "funcs.hpp"
#include "utils.hpp"
#include "status.hpp"
#include "CLI11.hpp"
#include <iostream>

int main(int argc, char** argv) {

  CLI::App app{"Dragonfly Narrowband core functions"};

  app.require_subcommand(1);

  // ---------------

  auto sub_cool = app.add_subcommand("cool", "Functions related to cooling and temperatures.");
  sub_cool->require_subcommand(1);

  sub_cool->add_subcommand("disable", "Turns off cooling.");
  sub_cool->add_subcommand("get", "Get the current temperatures for various parts of the system.");

  auto sub_cool_set = sub_cool->add_subcommand("set", "Enables cooling and sets the target cooling temperature.");

  float target_temp;
  sub_cool_set->add_option("temp", target_temp, "Target temperature in degrees C.")->required()->take_first();

  // ----------------

  auto sub_expose = app.add_subcommand("expose", "Take an exposure.");

  float duration;
  sub_expose->add_option("--duration", duration, "Duration of exposure in seconds.")->required();

  std::string filepath;
  sub_expose->add_option("--file", filepath, "Location to save exposure to.")->required();

  bool dark{false};
  sub_expose->add_flag("--dark", dark, "Take a dark instead of a light frame.");
  
  int bin_x = 1;
  sub_expose->add_option("--binx", bin_x, "Amount of binning for the x axis. Defaults to 1.");

  int bin_y = 1;
  sub_expose->add_option("--biny", bin_y, "Amount of binning for the y axis. Defaults to 1.");

  // ------------------
  
  CLI11_PARSE(app, argc, argv);

  // ------------------

	auto gateway = initialize_gateway();
  auto camera = unwrap_or_fail(initialize_camera(gateway));

  if (app.got_subcommand("cool")) {
    auto cooler = unwrap_or_fail(initialize_cooler(camera));

    if (sub_cool->got_subcommand("disable")) {
      disable_cooler(cooler);
      std::cout << "Disabling cooler." << std::endl;
    } else if (sub_cool->got_subcommand("get")) {
      std::cout << get_temp_info(camera, cooler) << std::endl;
    } else if (sub_cool->got_subcommand("set")) {
      auto sensor = unwrap_or_fail(initialize_sensor(camera));
      float tgt = set_temp(cooler, sensor, target_temp);
      std::cout << "Setting temperature to " << tgt << " degrees C." << std::endl;
    }
  } 

  if (app.got_subcommand("expose")) {

    auto sensor = unwrap_or_fail(initialize_sensor(camera));

    ExposureInfo expinfo;
    expinfo.bin_x = bin_x;
    expinfo.bin_y = bin_y;
    expinfo.duration = duration;
    expinfo.light = !dark;
    expinfo.readout_mode = ReadoutMode::Medium;

    std::cout << "Exposure in progress..." << std::endl;
    ExposeResult im = unwrap_or_fail(expose(camera, sensor, expinfo));
    std::cout << "Exposure complete" << std::endl;
    std::cout << "Saving image buffer to " << filepath << std::endl; 
    save_image(im, filepath.c_str());
  }

	free_gateway(gateway);
}
