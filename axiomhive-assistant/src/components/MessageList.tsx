import React from 'react';
import './MessageList.css';

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: Date;
}

interface MessageListProps {
  messages: Message[];
}

const MessageList: React.FC<MessageListProps> = ({ messages }) => {
  return (
    <div className="message-list">
      {messages.map((message) => (
        <div
          key={message.id}
          className={`message ${message.role === 'user' ? 'user-message' : 'assistant-message'}`}
        >
          <div className="message-content">
            {message.content}
          </div>
          <div className="message-timestamp">
            {message.timestamp.toLocaleTimeString()}
          </div>
        </div>
      ))}
    </div>
  );
};

export default MessageList;