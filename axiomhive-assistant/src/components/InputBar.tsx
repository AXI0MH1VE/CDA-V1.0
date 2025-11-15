import React, { useState, KeyboardEvent } from 'react';
import './InputBar.css';

interface InputBarProps {
  onSendMessage: (message: string) => void;
  disabled?: boolean;
  placeholder?: string;
}

const InputBar: React.FC<InputBarProps> = ({
  onSendMessage,
  disabled = false,
  placeholder = "Type your message..."
}) => {
  const [input, setInput] = useState('');

  const handleSubmit = () => {
    if (input.trim() && !disabled) {
      onSendMessage(input);
      setInput('');
    }
  };

  const handleKeyPress = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  return (
    <div className="input-bar">
      <div className="input-container">
        <textarea
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder={placeholder}
          disabled={disabled}
          rows={1}
          className="message-input"
        />
        <button
          onClick={handleSubmit}
          disabled={!input.trim() || disabled}
          className="send-button"
        >
          {disabled ? '...' : 'Send'}
        </button>
      </div>
    </div>
  );
};

export default InputBar;