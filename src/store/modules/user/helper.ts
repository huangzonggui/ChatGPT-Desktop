import * as _ from 'lodash'
import { ss } from '@/utils/storage'

const LOCAL_NAME = 'userStorage'

export interface UserInfo {
  avatar: string
  name: string | null
}

export interface UserConfig {
  modelName: string
  apiKey: string
  host: string
  proxy: string | null
  maxTokenNum: number
}

export interface UserState {
  userInfo: UserInfo
  userConfig: UserConfig
}

export function defaultSetting(): UserState {
  return {
    userInfo: {
      avatar: '',
      name: 'Tom',
    },
    userConfig: {
      modelName: 'gpt-3.5-turbo',
      apiKey: import.meta.env.VITE_GLOB_OPENAI_KEY,
      proxy: 'socks5://127.0.0.1:7890',
      // host: 'https://api.openai.com/v1/chat/completions',
      host: 'https://bypass.duti.tech/api/conversation',
      maxTokenNum: 4096,
    },
  }
}

export function allModels(): string[] {
  return ['gpt-3.5-turbo', 'gpt-3.5-turbo-0301', 'gpt-4', 'gpt-4-0314', 'gpt-4-32k', 'gpt-4-32k-0314']
}

export function getLocalState(): UserState {
  const localSetting: UserState | undefined = ss.get(LOCAL_NAME)
  const ds = defaultSetting()
  const state = _.merge(ds, localSetting)
  return state
}

export function setLocalState(setting: UserState): void {
  ss.set(LOCAL_NAME, setting)
}
