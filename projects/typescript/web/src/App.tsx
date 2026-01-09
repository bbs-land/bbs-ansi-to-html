import { useState, useCallback } from 'react';
import { ansiToHtml, type ConvertOptions } from '@bbs/ansi-to-html';
import { AnsiPreview } from './components/AnsiPreview.tsx';

interface FormState {
  file: File | null;
  utf8Input: boolean;
  synchronetCtrlA: boolean;
  renegadePipe: boolean;
}

function App() {
  const [formState, setFormState] = useState<FormState>({
    file: null,
    utf8Input: false,
    synchronetCtrlA: false,
    renegadePipe: false,
  });
  const [htmlOutput, setHtmlOutput] = useState<string>('');
  const [error, setError] = useState<string>('');


  // NOTE: ANSI component styles are provided by the site stylesheet
  // (`/style.css` in the repository `wwwroot/`). No runtime injection.

  const handleFileChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0] ?? null;
    setFormState(prev => ({ ...prev, file }));
    setError('');
    setHtmlOutput('');
  }, []);

  const handleCheckboxChange = useCallback((field: keyof FormState) => {
    return (e: React.ChangeEvent<HTMLInputElement>) => {
      setFormState(prev => ({ ...prev, [field]: e.target.checked }));
    };
  }, []);

  const handleConvert = useCallback(async () => {
    if (!formState.file) {
      setError('Please select a file to convert.');
      return;
    }

    try {
      setError('');

      // Read file as binary
      const arrayBuffer = await formState.file.arrayBuffer();
      const bytes = new Uint8Array(arrayBuffer);

      // Convert byte array to string where each char's charCode is the byte value
      let input: string;
      if (formState.utf8Input) {
        // For UTF-8 mode, decode as UTF-8 text
        const decoder = new TextDecoder('utf-8');
        input = decoder.decode(bytes);
      } else {
        // For CP437 mode, each byte becomes a character with that charCode
        input = Array.from(bytes, b => String.fromCharCode(b)).join('');
      }

      const options: ConvertOptions = {
        utf8Input: formState.utf8Input,
        synchronetCtrlA: formState.synchronetCtrlA,
        renegadePipe: formState.renegadePipe,
      };

      const html = ansiToHtml(input, options);
      setHtmlOutput(html);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An error occurred during conversion.');
    }
  }, [formState]);

  return (
    <div className="app-container">
      <h1>ANSI to HTML Converter</h1>

      <section className="upload-form">
        <div className="form-group">
          <label htmlFor="ansi-file">Select ANSI Art File</label>
          <input
            type="file"
            id="ansi-file"
            accept=".msg,.ans,.txt"
            onChange={handleFileChange}
          />
        </div>

        <div className="options-group">
          <label className="checkbox-label">
            <input
              type="checkbox"
              checked={formState.utf8Input}
              onChange={handleCheckboxChange('utf8Input')}
            />
            UTF-8 Input
          </label>

          <label className="checkbox-label">
            <input
              type="checkbox"
              checked={formState.synchronetCtrlA}
              onChange={handleCheckboxChange('synchronetCtrlA')}
            />
            Synchronet Ctrl-A Codes
          </label>

          <label className="checkbox-label">
            <input
              type="checkbox"
              checked={formState.renegadePipe}
              onChange={handleCheckboxChange('renegadePipe')}
            />
            Renegade Pipe Codes
          </label>
        </div>

        <button
          type="button"
          className="convert-button"
          onClick={handleConvert}
          disabled={!formState.file}
        >
          Convert &amp; View
        </button>

        {error && <div className="error-message">{error}</div>}
      </section>

      {htmlOutput && (
        <section className="preview-section">
          <div className="preview-header">Preview</div>
          <div className="preview-content">
            <AnsiPreview html={htmlOutput} />
          </div>
        </section>
      )}

      <p className="help-text">
        Supported formats: .msg, .ans (ANSI art files with CP437 encoding)
      </p>
    </div>
  );
}

export default App;
