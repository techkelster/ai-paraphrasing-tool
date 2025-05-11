import { useState, useRef, useEffect } from 'react';
import axios from 'axios';

const TextEditor = () => {
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const editorRef = useRef<HTMLDivElement>(null);
  
  const handleTextChange = (e: React.FormEvent<HTMLDivElement>) => {
    // We use the contentEditable div directly, no need to track text in state
    // The content is accessed via editorRef when needed
  };

  const getSelectedText = (): { text: string, range: Range } | null => {
    const selection = window.getSelection();
    if (!selection || selection.rangeCount === 0 || selection.toString().trim() === '') {
      return null;
    }
    
    const range = selection.getRangeAt(0);
    const selectedText = selection.toString();
    
    return { text: selectedText, range };
  };

  const paraphraseSelectedText = async () => {
    const selection = getSelectedText();
    if (!selection) {
      setError('Please select some text to paraphrase');
      setTimeout(() => setError(null), 3000);
      return;
    }

    setIsLoading(true);
    setError(null);
    
    try {
      const response = await axios.post('http://localhost:8080/api/paraphrase', {
        text: selection.text
      });
      
      // Replace the selected text with the paraphrased version
      if (response.data.paraphrased) {
        const range = selection.range;
        range.deleteContents();
        range.insertNode(document.createTextNode(response.data.paraphrased));
        
        // No need to update state, the contentEditable div is updated directly
      }
    } catch (err) {
      setError('Failed to paraphrase text. Please try again.');
      console.error('Paraphrase error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  // Show the paraphrase tooltip when text is selected
  const [showTooltip, setShowTooltip] = useState(false);
  const [tooltipPosition, setTooltipPosition] = useState({ top: 0, left: 0 });
  
  useEffect(() => {
    const checkSelection = () => {
      const selection = window.getSelection();
      if (selection && selection.toString().trim() !== '') {
        const range = selection.getRangeAt(0);
        const rect = range.getBoundingClientRect();
        
        setTooltipPosition({
          top: rect.top - 40,
          left: rect.left + rect.width / 2
        });
        setShowTooltip(true);
      } else {
        setShowTooltip(false);
      }
    };

    document.addEventListener('mouseup', checkSelection);
    return () => {
      document.removeEventListener('mouseup', checkSelection);
    };
  }, []);

  useEffect(() => {
    // Set placeholder text when editor is empty
    if (editorRef.current) {
      editorRef.current.dataset.placeholder = "Type or paste your text here...";
    }
  }, []);

  return (
    <div className="relative w-full mx-auto">
      {error && (
        <div className="bg-red-50 border-l-4 border-red-500 text-red-800 p-4 rounded-md mb-4 shadow-sm flex items-start">
          <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5 mr-2 mt-0.5 text-red-500" viewBox="0 0 20 20" fill="currentColor">
            <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
          </svg>
          <span>{error}</span>
        </div>
      )}
      
      <div className="bg-white rounded-xl shadow-md overflow-hidden">
        <div className="bg-slate-50 px-4 py-2 border-b border-slate-200 flex items-center">
          <div className="flex space-x-1.5">
            <div className="w-3 h-3 rounded-full bg-red-400"></div>
            <div className="w-3 h-3 rounded-full bg-amber-400"></div>
            <div className="w-3 h-3 rounded-full bg-green-400"></div>
          </div>
          <span className="ml-2 text-xs text-slate-500">Editor</span>
        </div>
        
        <div
          ref={editorRef}
          contentEditable
          onInput={handleTextChange}
          className="min-h-[300px] p-6 focus:outline-none prose prose-slate max-w-none empty:before:content-[attr(data-placeholder)] empty:before:text-slate-400 empty:before:italic"
          data-placeholder="Type or paste your text here..."
        />
      </div>
      
      {showTooltip && (
        <div 
          className="absolute z-10 transform -translate-x-1/2 animate-fade-in"
          style={{ top: `${tooltipPosition.top}px`, left: `${tooltipPosition.left}px` }}
        >
          <button
            onClick={paraphraseSelectedText}
            disabled={isLoading}
            className="bg-gradient-to-r from-blue-500 to-indigo-600 hover:from-blue-600 hover:to-indigo-700 text-white px-4 py-2 rounded-full text-sm font-medium shadow-md transition-all duration-200 flex items-center space-x-2"
          >
            {isLoading ? (
              <>
                <svg className="animate-spin h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                <span>Paraphrasing...</span>
              </>
            ) : (
              <>
                <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z" />
                </svg>
                <span>Paraphrase</span>
              </>
            )}
          </button>
        </div>
      )}
      
      <div className="mt-5 text-center">
        <p className="text-sm text-slate-500">
          Select any text in the editor above to paraphrase it
        </p>
      </div>
    </div>
  );
};

export default TextEditor; 