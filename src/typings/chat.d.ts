declare namespace Chat {

	// 一条聊天消息
	interface Chat {
		dateTime: string
		text: string
		tokenNum?: number
		inversion?: boolean
		error?: boolean
		loading?: boolean
		conversationOptions?: ConversationRequest | null
		requestOptions: { prompt: string; options?: ConversationRequest | null }
	}

	interface History {
		title: string
		isEdit: boolean
		uuid: number
	}

	interface ChatOptions {
		apiKey: string
		accessToken: string
		accessType: string
		proxy: string | null
		model: string
		systemMessage: string
		temperature: number
	}

	// 一个聊天会话
	interface ChatSession {
		uuid: number
		data: Chat[]
		opt: Partial<ChatOptions>
	}

	interface ChatState {
		active: number | null
		history: History[]
		chat: ChatSession[]
	}

	interface ConversationRequest {
		conversationId?: string
		parentMessageId?: string
	}


	interface RequestMessage {
		role: string
		content: string
	}
}
