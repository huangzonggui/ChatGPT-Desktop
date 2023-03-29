import { defineStore } from 'pinia'
import type { UserInfo, UserState } from './helper'
import { allModels, defaultSetting, getLocalState, setLocalState } from './helper'

export const useUserStore = defineStore('user-store', {
  state: (): UserState => getLocalState(),
  actions: {
    updateUserInfo(userInfo: Partial<UserInfo>) {
      this.userInfo = { ...this.userInfo, ...userInfo }
      this.recordState()
    },

    addApiKey(apiKey: string) {
      this.userConfig.apiKeyList.push(apiKey)
    },

    deleteApiKey(apiKey: string) {
      this.userConfig.apiKeyList = this.userConfig.apiKeyList.filter(item => item !== apiKey)
      this.recordState()
    },

    addAccessToken(accessToken: string) {
      this.userConfig.accessTokenList.push(accessToken)
    },

    deleteAccessToken(accessToken: string) {
      this.userConfig.accessTokenList = this.userConfig.accessTokenList.filter(item => item !== accessToken)
      this.recordState()
    },

    addHost(host: string) {
      this.userConfig.hostList.push(host)
      this.recordState()
    },

    deleteHost(host: string) {
      this.userConfig.hostList = this.userConfig.hostList.filter(item => item !== host)
      this.recordState()
    },

    resetHost() {
      this.userConfig.hostList = defaultSetting().userConfig.hostList
      this.recordState()
    },

    resetUserInfo() {
      this.userInfo = { ...defaultSetting().userInfo }
      this.recordState()
    },

    recordState() {
      setLocalState(this.$state)
    },

    allModels(): string[] {
      return allModels()
    },
  },
})
