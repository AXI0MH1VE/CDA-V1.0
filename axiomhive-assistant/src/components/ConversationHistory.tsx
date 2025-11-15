import React from 'react';
import './ConversationHistory.css';

interface Conversation {
  id: string;
  title: string;
}

interface ConversationHistoryProps {
  conversations: Conversation[];
  onSelectConversation: (id: string) => void;
  className?: string;
}

const ConversationHistory: React.FC<ConversationHistoryProps> = ({
  conversations,
  onSelectConversation,
  className,
}) => {
  // Dummy data for now
  const dummyConversations: Conversation[] = [
    { id: '1', title: 'First Conversation' },
    { id: '2', title: 'Quantum Computing Explained' },
    { id: '3', title: 'Python Code Review' },
  ];

  return (
    <div className={`history-panel ${className}`}>
      <h2>History</h2>
      <ul className="history-list">
        {dummyConversations.map(convo => (
          <li key={convo.id} onClick={() => onSelectConversation(convo.id)}>
            {convo.title}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default ConversationHistory;
