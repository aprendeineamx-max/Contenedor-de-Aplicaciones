import {
  Configuration,
  ContainersApi,
  SystemApi,
  TasksApi,
} from '@orbit/panel-sdk';

export async function fetchSystemConfig(baseUrl: string, token: string) {
  const normalizedBase = baseUrl.replace(/\/$/, '');
  const configuration = new Configuration({
    basePath: normalizedBase,
    accessToken: token,
  });
  const api = new SystemApi(configuration);
  return api.systemConfigGet();
}

export async function fetchContainers(baseUrl: string, token: string) {
  const configuration = new Configuration({
    basePath: baseUrl.replace(/\/$/, ''),
    accessToken: token,
  });
  const api = new ContainersApi(configuration);
  return api.containersGet();
}

export async function fetchTasks(
  baseUrl: string,
  token: string,
  limit = 10,
) {
  const configuration = new Configuration({
    basePath: baseUrl.replace(/\/$/, ''),
    accessToken: token,
  });
  const api = new TasksApi(configuration);
  return api.tasksGet({ limit });
}

