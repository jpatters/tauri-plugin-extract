import { invoke } from '@tauri-apps/api/tauri'

export async function extract(srcZip: string, targetDir: string) {
  return await invoke('plugin:extract|extract', {})
}
