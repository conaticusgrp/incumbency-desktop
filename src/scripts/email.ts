export interface EmailUser {
    username: string,
    address?: string
}

export interface EmailData {
    title: string,
    content: string,
    date: string,
    sender?: EmailUser,
    cc?: EmailUser
}
