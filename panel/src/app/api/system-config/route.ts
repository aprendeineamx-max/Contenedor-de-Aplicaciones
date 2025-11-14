import { NextResponse } from 'next/server';
import { fetchSystemConfig } from '@/lib/orbit';

export async function POST(request: Request) {
  const { baseUrl, token } = await request.json();

  if (!baseUrl || !token) {
    return NextResponse.json(
      { error: 'baseUrl y token son obligatorios' },
      { status: 400 }
    );
  }

  try {
    const snapshot = await fetchSystemConfig(baseUrl, token);
    return NextResponse.json(snapshot);
  } catch (error) {
    console.error('No se pudo consultar /system/config', error);
    return NextResponse.json({ error: 'No se pudo consultar /system/config' }, { status: 502 });
  }
}

