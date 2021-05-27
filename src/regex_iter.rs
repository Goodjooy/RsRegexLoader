pub struct RegexIter {
    data: String,
    data_vec: Vec<usize>,
    index: usize,
}

impl RegexIter {
    pub fn new(regex: &str) -> RegexIter {
        let data = String::from(regex);

        let data_vec = data.char_indices().map(|(i, _v)| -> usize { i }).collect();

        RegexIter {
            data: String::from(regex),
            index: 0,
            data_vec,
        }
    }

    pub fn per_look<'a>(&'a self, len: isize) -> Option<&str> {
        let (start_index, end_index) = match self.get_follow_range(len as usize) {
            Some((s, e)) => (s, e),
            None => return None,
        };

        if start_index >= self.data_vec.len() || end_index > self.data_vec.len() {
            None
        } else {
            Some(&self.data[start_index..end_index])
        }
    }
}

impl Iterator for RegexIter {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        if self.index >= self.data_vec.len() {
            None
        } else {
            match self.get_next_index() {
                Some((s, e)) => {
                    self.index += 1;
                    self.data[s..e].chars().next()
                }
                None => None,
            }
        }
    }
}

impl RegexIter {
    pub fn has_next(&self) -> bool {
        self.index < self.data_vec.len()
    }

    fn get_next_index(&mut self) -> Option<(usize, usize)> {
        if !self.has_next() {
            None
        } else {
            let s_index = self.index;
            let e_index = self.index + 1;
            let len:usize=
            //索引超出限制
            if e_index>=self.data_vec.len(){
                self.data.len()-self.data_vec[s_index]
            }else{
                self.data_vec[e_index]-self.data_vec[s_index]
            };

            Some((self.data_vec[s_index], self.data_vec[s_index] + len))
        }
    }

    fn get_follow_range(&self, len: usize) -> Option<(usize, usize)> {
        if self.index + len > self.data_vec.len() {
            return None;
        } else {
            let mut totle_len: usize = 0;
            let mut len = len;
            if self.index + len == self.data_vec.len() {
                totle_len += match self.data_vec.last() {
                    Some(u) => self.data.len() - u,
                    None => 0,
                };
                len -= 1;
            }

            totle_len += self.data_vec[self.index+len]  - self.data_vec[self.index];
            Some((
                self.data_vec[self.index],
                self.data_vec[self.index] + totle_len,
            ))
        }
    }
}
