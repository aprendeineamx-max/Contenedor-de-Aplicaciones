import { Configuration, SystemApi } from '@orbit/panel-sdk';

export async function fetchSystemConfig(baseUrl: string, token: string) {
  const normalizedBase = baseUrl.replace(/\/$/, '');
  const configuration = new Configuration({
    basePath: normalizedBase,
    accessToken: token,
  });
  const api = new SystemApi(configuration);
  return api.systemConfigGet();
}

