<script setup lang='ts'>
import { computed, ref } from 'vue'
import type { FormInst, FormItemRule, FormRules } from 'naive-ui'
import { NButton, NForm, NFormItem, NInput, NRadio, NRadioGroup, NSelect, useMessage } from 'naive-ui'
import { useUserStore } from '@/store'
import { t } from '@/locales'
import { genOptionByList } from '@/utils/functions'

const userStore = useUserStore()
const ms = useMessage()
const formRef = ref<FormInst | null>(null)
const userInfo = computed(() => userStore.userInfo)
const userConfig = computed(() => userStore.userConfig)

const model = ref({
  name: userInfo.value.name,
  avatar: userInfo.value.avatar,
  apiKey: userConfig.value.apiKey,
  accessToken: userConfig.value.accessToken,
  apiKeyList: userConfig.value.apiKeyList,
  accessTokenList: userConfig.value.accessTokenList,
  hostList: userConfig.value.hostList,
  modelName: userConfig.value.modelName,
  host: userConfig.value.host,
  proxy: userConfig.value.proxy,
  accessType: userConfig.value.accessType,
})

const models = genOptionByList(userStore.allModels())

const rules: FormRules = {
  name: [
    {
      required: true,
      message: t('setting.namePlaceholder'),
      validator(rule: FormItemRule, value: string) {
        if (!value)
          return new Error(t('setting.nameNotEmptyError'))

        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
  proxy: [{
    required: false,
    validator(rule: FormItemRule, value: string) {
      if (!value || value.length === 0)
        return true

      else if (!/^(socks5):\/\/.+$/.test(value))
        return new Error('Proxy must start with socks5://')
      return true
    },
    trigger: ['input', 'blur'],
  }],
  apiKey: [
    {
      message: '请输入 api-key',
      validator(rule: FormItemRule, value: string) {
        if (!value)
          return new Error('不能为空')
        // else if (!/^sk-\w+$/.test(value))
        //   return new Error('请输入正确的api-key')

        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
  host: [
    {
      required: true,
      message: '请输入openai api host',
      validator(rule: FormItemRule, value: string) {
        if (!value)
          return new Error('不能为空')
        else if (!/^https:\/\/\S+$/.test(value))
          return new Error('请输入正确的host')

        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
  accessType: [
    {
      required: true,
      message: '选择访问方式',
      validator(rule: FormItemRule, value: string) {
        if (!value)
          return new Error('不能为空')
        return true
      },
    },
  ],
}

function saveUserInfo() {
  formRef.value?.validate((errors) => {
    if (!errors) {
      userInfo.value.name = model.value.name
      userInfo.value.avatar = model.value.avatar
      userConfig.value.apiKey = model.value.apiKey
      userConfig.value.accessToken = model.value.accessToken
      userConfig.value.modelName = model.value.modelName
      userConfig.value.proxy = model.value.proxy
      userConfig.value.host = model.value.host
      userConfig.value.accessType = model.value.accessType
      userStore.recordState()
      ms.success(t('common.success'))
    }
  })
}
</script>

<template>
  <div class="p-4 space-y-5 overflow-auto">
    <NForm ref="formRef" :model="model" :rules="rules">
      <NFormItem path="avatar" :label="$t('setting.avatarLink')">
        <NInput v-model:value="model.avatar" :placeholder="$t('setting.avatarLinkPlaceholder')" />
      </NFormItem>
      <NFormItem path="name" :label="$t('setting.name')">
        <NInput v-model:value="model.name" :placeholder="$t('setting.namePlaceholder')" />
      </NFormItem>
      <NFormItem path="type" label="访问方式">
        <NRadioGroup v-model:value="model.accessType">
          <NRadio value="0">
            API key
          </NRadio>
          <NRadio value="1">
            Access Token
          </NRadio>
        </NRadioGroup>
      </NFormItem>
      <NFormItem path="apiKey" label="Openai API Key">
        <NSelect v-model:value="model.apiKey" placeholder="Select" :options="genOptionByList(model.apiKeyList)" />
      </NFormItem>
      <NFormItem path="accessToken" label="Access Token">
        <NSelect v-model:value="model.accessToken" placeholder="Select" :options="genOptionByList(model.accessTokenList)" />
      </NFormItem>
      <NFormItem path="host" label="Host">
        <NSelect v-model:value="model.host" placeholder="Select" :options="genOptionByList(model.hostList)" />
      </NFormItem>
      <NFormItem path="modelName" label="Model Name">
        <NSelect v-model:value="model.modelName" placeholder="Select" :options="models" />
      </NFormItem>
      <NFormItem path="proxy" label="Proxy">
        <NInput v-model:value="model.proxy" placeholder="socks5://127.0.0.1:7890" />
      </NFormItem>
      <div class="flex items-center justify-end">
        <NButton size="small" @click="saveUserInfo">
          {{ $t('setting.saveUserInfoBtn') }}
        </NButton>
      </div>
    </NForm>
  </div>
</template>
