'use server';

import { revalidatePath } from 'next/cache';
import { createContainer, deleteContainer } from '@/lib/orbit';

const baseUrl = process.env.ORBIT_PANEL_BASE_URL;
const token = process.env.ORBIT_PANEL_TOKEN;

function ensureEnv() {
  if (!baseUrl || !token) {
    throw new Error('Define ORBIT_PANEL_BASE_URL y ORBIT_PANEL_TOKEN en panel/.env.local para usar acciones.');
  }
  return { baseUrl, token };
}

export async function createContainerAction(formData: FormData) {
  const name = String(formData.get('name') ?? '').trim();
  const platform = String(formData.get('platform') ?? '').trim();
  if (!name || !platform) {
    throw new Error('Nombre y plataforma son obligatorios');
  }
  const env = ensureEnv();
  await createContainer(env.baseUrl, env.token, name, platform);
  revalidatePath('/');
}

export async function deleteContainerAction(formData: FormData) {
  const id = String(formData.get('containerId') ?? '').trim();
  if (!id) {
    throw new Error('ID obligatorio');
  }
  const env = ensureEnv();
  await deleteContainer(env.baseUrl, env.token, id);
  revalidatePath('/');
}
