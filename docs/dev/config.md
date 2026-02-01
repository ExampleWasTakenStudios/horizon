# Configuration

This section only refers to the configuration of the application. It does not cover configuring DNS related settings.

At the center of Horizon's configuration is the ConfigManager class that manages the config file.

## Behavior

### Startup

Upon startup, the config manager is initialized and checks if a config file exists. If negative, it creates one with default settings defined in [DefaultConfig.ts](../../src/config/DefaultConfig.ts).

Once a config file has either been created or read the configuration data is loaded to memory and exposed via a getter method. All classes wishing to access the config data, must use the config manager instance through dependency injection.

> [!NOTE]
> Currently, the config is loaded once at startup and never thereafter! Hot reloading functionality is on the roadmap and will be implemented at a later point.

### File paths

The config is stored in different locations based on the NODE_ENV env var:

- `dev`: `process.cwd()/.config/Horizon/config.json5`
- `prod`: `etc/opt/Horizon/config.json5`

---

##### Note to devs implementing hot reloading

The following requirements exist for hot reloading functionality:

1. The process must be fully atomic and asynchronous.
2. Any failures occurring during hot reloading MUST be handled and result in the previous config being used. -> Crashes due to invalid configs are not acceptable.
