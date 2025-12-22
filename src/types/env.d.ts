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
    }
  }
}

export {};
