def add_array(n,x)
  a = Array.new(n, 0)
  x.times do
    for i in 0..x-1
      a[i] += 1
    end
  end
  a.sum
end

puts add_array(ARGV[0].to_i, ARGV[1].to_i)
