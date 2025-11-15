import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import MessageList from './MessageList';
import InputBar from './InputBar';
import './ChatInterface.css';

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: Date;
}

const ChatInterface: React.FC = () => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    // Add constitutional disclosure on first load
    if (messages.length === 0) {
      const disclosureMessage: Message = {
        id: 'disclosure',
        role: 'assistant',
        content: "I am an AI, a computational tool. I do not have consciousness, feelings, or a personal identity. I am bound by the Constitution of a Deterministic Assistant (CDA-v1.0) to ensure transparency, safety, and deterministic operation.",
        timestamp: new Date(),
      };
      setMessages([disclosureMessage]);
    }
  }, []);

  const handleSendMessage = async (content: string) => {
    if (!content.trim()) return;

    const userMessage: Message = {
      id: `user-${Date.now()}`,
      role: 'user',
      content: content.trim(),
      timestamp: new Date(),
    };

    setMessages(prev => [...prev, userMessage]);
    setIsLoading(true);

    try {
      const response = await invoke<string>('process_query', { query: content });

      const assistantMessage: Message = {
        id: `assistant-${Date.now()}`,
        role: 'assistant',
        content: response,
        timestamp: new Date(),
      };

      setMessages(prev => [...prev, assistantMessage]);
    } catch (error) {
      console.error('Error sending message:', error);
      const errorMessage: Message = {
        id: `error-${Date.now()}`,
        role: 'assistant',
        content: 'Sorry, I encountered an error processing your request.',
        timestamp: new Date(),
      };
      setMessages(prev => [...prev, errorMessage]);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="chat-interface">
      <div className="chat-header">
        <h1>AxiomHive Assistant</h1>
        <p>Constitutionally compliant AI assistant</p>
      </div>

      <MessageList messages={messages} />

      <InputBar
        onSendMessage={handleSendMessage}
        disabled={isLoading}
        placeholder={isLoading ? "Processing..." : "Type your message..."}
      />
    </div>
  );
};

export default ChatInterface;