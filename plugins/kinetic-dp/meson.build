if get_option('plugin_kinetic_dp').require(enable_udev,
    error_message: 'udev is needed for plugin_kinetic_dp').allowed()
cargs = ['-DG_LOG_DOMAIN="FuPluginKineticDp"']
plugins += {meson.current_source_dir().split('/')[-1]: true}

plugin_builtin_kinetic_dp = static_library('fu_plugin_kinetic_dp',
  rustgen.process('fu-kinetic-dp.rs'),
  sources: [
    'fu-kinetic-dp-device.c',
    'fu-kinetic-dp-plugin.c',
    'fu-kinetic-dp-puma-device.c',
    'fu-kinetic-dp-puma-firmware.c',
    'fu-kinetic-dp-secure-device.c',
    'fu-kinetic-dp-secure-firmware.c',
  ],
  include_directories: plugin_incdirs,
  c_args: [
    cargs,
  ],
  link_with: plugin_libs,
  dependencies: plugin_deps,
)
plugin_builtins += plugin_builtin_kinetic_dp
endif
