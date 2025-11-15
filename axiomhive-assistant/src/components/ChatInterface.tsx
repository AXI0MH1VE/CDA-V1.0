import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import MessageList from './MessageList';
import InputBar from './InputBar';
import ConversationHistory from './ConversationHistory';
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
  const [isHistoryVisible, setIsHistoryVisible] = useState(true);

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
      <ConversationHistory
        conversations={[]}
        onSelectConversation={() => {}}
        className={isHistoryVisible ? '' : 'collapsed'}
      />
      <div className="chat-main">
        <div className="chat-header">
          <button className="history-toggle-btn" onClick={() => setIsHistoryVisible(!isHistoryVisible)}>
            {isHistoryVisible ? '‹' : '›'}
          </button>
          <h1>AxiomHive Assistant</h1>
        </div>

        <MessageList messages={messages} />

        <InputBar
          onSendMessage={handleSendMessage}
          disabled={isLoading}
          placeholder={isLoading ? "Processing..." : "Type your message..."}
        />
      </div>
    </div>
  );
};

export default ChatInterface;