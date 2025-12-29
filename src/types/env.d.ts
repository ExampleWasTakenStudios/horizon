declare global {
  namespace NodeJS {
    interface ProcessEnv {
      /**
       * Indicates the environment in which the application is running.
       *
       * `dev` - Indicates a development environment
       *
       * `prod` - Indicates a production environment.
       */
      NODE_ENV: 'dev' | 'prod';
      HORIZON_CONSOLE_LOG_LEVEL: 0 | 1 | 2 | 3 | 4 | 5;
      HORIZON_FILE_LOG_LEVEL: 0 | 1 | 2 | 3 | 4 | 5;
    }
  }
}

export {};
