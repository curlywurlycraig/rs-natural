use std::collections::HashMap;
use stem;
use tokenize::tokenize;

pub struct NaiveBayesClassifier {
  documents: HashMap<String, HashMap<String, usize>>,
  total_document_count: usize,
}

impl NaiveBayesClassifier {
  pub fn new() -> NaiveBayesClassifier {
    NaiveBayesClassifier {
      documents: HashMap::new(),
      total_document_count: 0,
    }
  }
  
  // Add counts of terms in some text to a classification
  pub fn train(&mut self, text: &str, classification: &str) {
    let classification_map = self.documents.entry(classification.to_string())
                                           .or_insert(HashMap::new());
    let stemmed_and_tokenized = get_tokenized_and_stemmed(text);
    for stemmed_word in stemmed_and_tokenized.into_iter() {
      let stemmed_word_entry = classification_map.entry(stemmed_word).or_insert(1);
      *stemmed_word_entry += 1;
    }
    self.total_document_count += 1;
  }
  
  // Get a guess of input text based on existing unigram counts
  pub fn guess(&self, text: &str) -> String {
    let stemmed_and_tokenized = get_tokenized_and_stemmed(text);
    let mut label_probabilities = HashMap::new();

    for (classification, word_counts) in self.documents.iter() {
      //Get the probability that the passed-in text is each class
      let mut probability = 0.0f32;
      for stemmed_word in &stemmed_and_tokenized {
        if word_counts.contains_key(stemmed_word) {
          probability += (1.0 / word_counts.len() as f32).ln();
        }
      }

      // store the calculated probability for the classification
      if probability.abs() < 0.0001 {
        label_probabilities.insert(classification, 0.0);
      } else {
        let normalized_prob = word_counts.len() as f32 * probability.abs() /
                              self.total_document_count as f32;

        label_probabilities.insert(classification, normalized_prob);
      }
    }
    
    // determine the label of the highest probability
    let mut result_label = String::new();
    let mut result_probability = 0.0;
    for (classification, prob) in label_probabilities.into_iter() {
      if result_probability <= prob {
        result_label = classification.clone();
        result_probability = prob;
      }
    }
    result_label
  }
}

fn get_tokenized_and_stemmed(text: &str) -> Vec<String> {
  let tokenized_text = tokenize(text);
  tokenized_text.into_iter()
                .map(|text| stem::get(text).unwrap())
                .collect()
}
