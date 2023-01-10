import { invoke } from '@tauri-apps/api/tauri'

export async function extract(opts: {srcZip: string, outDir: string}) {
  return await invoke('plugin:extract|extract', opts)
}
